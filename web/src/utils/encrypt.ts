import NodeRSA from 'node-rsa';

export function encrypt(plaintext: string): string {
  // Example usage
  const key = new NodeRSA(`-----BEGIN PUBLIC KEY-----
MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAvnADloWov6qNmg0f7cpX
qDWbI/uhqK0ZzYsyKIRG8WEFVAvumDVbSjMuvxLOzrquU4EEhO6M5SvT2DT1RwqP
aDch07PLgND6OoH/T4J+pjep/0z2DxjXH5yJIG6KwkiqROgjneb35CazAKFfLZvi
hB/1y9M4MG3Wp0g/t/CRf/g0qZ0WV59L8Hd3IYgLl4DhQmYx2Tb4Twe5n2fbwVja
kKQb28Yqnoig7aYU7Z8kZa0n+Nr9PV+FGHBnguS5fu+7wO/X8tGwFXT2ASQJNcRh
F6JYJzVuB7dZ3JAH5ChzBPLJUNnD9g2YXZd69Vd9S8IRkb+YtrrAGNW3R0a0zktB
PAMqkWElcN9uYY+XX6vU/CNntEu+hK61mHmFq1pT2W2ndv1CqsOMWl6OCuTQBrYI
e6Bw7f0fFl8YDJITFb08tYqfMY/cIFB2oAW9IyNXXhtWVi3A/fRt8Acv/fLMfga9
8MKibxD05s0grTyvUZkUG1xGPRWJ/E7JXHupuKRAv0WVQDzhZT8sdmZX59h9e9m7
7CDyXb0g0d9sVpxWfD6qNSRbKt8ty/cL78AE9hItQyMEJ5h3V+z8yYp3XjCwmwHK
mCNY4F9S4pHeg0X6AF5ajWQ7GLssRf6dpkSKuZul3HC3HWeEPLv0gPzhGv+J5EOB
9sblxlSYbn8Eupc0qRjn+wUCAwEAAQ==
-----END PUBLIC KEY-----`);

  return encryptedText;
}
