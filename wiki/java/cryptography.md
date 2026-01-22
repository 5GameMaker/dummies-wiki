# Cryptography[^2]

Java's cryptography revolves around the following classes[^1]:

```java
import java.security.Security;         // For defining and listing providers
import java.security.Provider;         // For providing encryption schemes

import java.security.KeyFactory;       // For parsing keys
import java.security.KeyPair;          // Keys as application state
import java.security.KeyPairGenerator; // For generating keys
import javax.crypto.Cipher;            // For actually encrypting stuff
```

Keys can be either generated at runtime:
```java
import java.security.KeyPairGenerator;
import java.security.NoSuchAlgorithException;

try {
    KeyPairGenerator gen = KeyPairGenerator.getInstance("RSA");
    gen.initialize(4096);
    KeyPair pair = gen.generateKeyPair(); // Caution! This method takes a long time to execute!
} catch (NoSuchAlgorithException error) {
    // ...
}
```
...or loaded from a source:
```java
import java.security.KeyFactory;
import java.security.NoSuchAlgorithException;
import java.security.spec.X509EncodedKeySpec;
import java.security.spec.PKCS8EncodedKeySpec;
import java.security.spec.InvalidKeySpecException;

// Inputs
byte[] pubkeyBytes  = ...;
byte[] privkeyBytes = ...;

try {
    KeyFactory factory = KeyFactory.getInstance("RSA");
    KeyPair pair = new KeyPair(
        factory.generatePublic(new X509EncodedKeySpec(pubkeyBytes)),
        factory.generatePrivate(new PKCS8EncodedKeySpec(privkeyBytes))
    );
} catch (NoSuchAlgorithException | InvalidKeySpecException error) {
    // ...
}
```
,...and then saved to a sink:
```java
import java.security.KeyPair;
import java.security.spec.X509EncodedKeySpec;
import java.security.spec.PKCS8EncodedKeySpec;

// Input
KeyPair pair = ...;

byte[] pubkeyBytes  = new X509EncodedKeySpec(pair.getPublic().getEncoded()).getEncoded();
byte[] privkeyBytes = new PKCS8EncodedKeySpec(pair.getPrivate().getEncoded()).getEncoded();

// Alternatively

import java.security.KeyPair;
import java.security.NoSuchAlgorithException;
import java.security.spec.X509EncodedKeySpec;
import java.security.spec.PKCS8EncodedKeySpec;
import java.security.spec.InvalidKeySpecException;

// Input
KeyPair pair = ...;

try {
    KeyFactory factory  = KeyFactory.getInstance("RSA");
    byte[] pubkeyBytes  = factory.getKeySpec(pair.getPublic(), X509EncodedKeySpec.class).getEncoded();
    byte[] privkeyBytes = factory.getKeySpec(pair.getPrivate(), PKCS8EncodedKeySpec.class).getEncoded();
} catch (NoSuchAlgorithException | InvalidKeySpecException error) {
    // ...
}
```

## Encrypting and decrypting

Encrypting and decrypting is done via `Cipher` class.

Importantly, Android breaks the default `RSA` scheme specifically for `Cipher`. Thus, you need to use `RSA/ECB/PKCS1Padding`
scheme for it instead[^3]. Using `RSA` cross-platform is going to fail with `BadPaddingException`.

```java
import java.security.PublicKey;
import java.security.PrivateKey;
import java.security.NoSuchAlgorithException;
import java.security.NoSuchPaddingException;
import java.security.IllegalBlockSizeException;
import javax.crypto.Cipher;
import javax.crypto.NoSuchPaddingException;

// Inputs
byte[] sourceData = ...;
PublicKey  pub    = ...; // (KeyPair) pair.getPublic();
PrivateKey priv   = ...; // (KeyPair) pair.getPrivate();

// Output
byte[] encryptedData;
try {
    Cipher cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
    cipher.init(Cipher.ENCRYPT_MODE, priv);
    encryptedData = cipher.doFinal(sourceData);
} catch (NoSuchAlgorithException | NoSuchPaddingException | IllegalBlockSizeException | NoSuchPaddingException error) {
    // ...
    return;
}

// Output
byte[] decryptedData;
try {
    Cipher cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
    cipher.init(Cipher.DECRYPT_MODE, pub);
    decryptedData = cipher.doFinal(encryptedData);
} catch (NoSuchAlgorithException | NoSuchPaddingException | IllegalBlockSizeException | NoSuchPaddingException error) {
    // ...
    return;
}

assert Arrays.equals(sourceData, decryptedData);
```

[^1]: https://www.baeldung.com/java-list-cipher-algorithms
[^2]: https://www.baeldung.com/java-rsa
[^3]: https://flutterfixes.com/rsa-encryption-disparity-between-android-and-java-environments/
