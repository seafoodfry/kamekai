# Kamekai Backend

```
make
```

[crates/aws-config](https://crates.io/crates/aws-config)

[crates/aws-sdk-sts](https://crates.io/crates/aws-sdk-sts)

[crates/aws-sdk-bedrockruntime](https://crates.io/crates/aws-sdk-bedrockruntime)

[serde](https://serde.rs/)

Testing code:

```
./run-cmd-in-shell.sh cargo run
```

```json
{
  "translations": [
    {
      "original": "I told you so",
      "japanese": {
        "translation": "だから言ったでしょう",
        "pronunciation": "dakara itta deshou",
        "grammar": [
          "だから (dakara): 'That's why' or 'So.'",
          "言った (いった, itta): The past tense of 言う (いう, iu), meaning “to say” or “to tell.”",
          "でしょう (deshou): A sentence-ending particle that adds a tone of confirmation or assertion, often implying “didn’t I?” or “right?”"
        ],
        "examples": [
          {
            "phrase": "ほら、だから言ったでしょう！",
            "pronunciation": "ほら、だからいったでしょう!",
            "translation": "See, I told you so!"
          },
          {
            "phrase": "言ったよね",
            "pronunciation": "いったよね",
            "translation": "I told you, right?"
          }
        ]
      },
      "chinese": {
        "translation": "我早就跟你说了",
        "pronunciation": "wǒ zǎo jiù gēn nǐ shuō le",
        "grammar": [
          "早就 (zǎo jiù): “A long time ago” or “already.”",
          "跟 (gēn): “With” or “to.”",
          "说了 (shuō le): “Said” or “told.”"
        ],
        "examples": [
          {
            "phrase": "我就说嘛",
            "pronunciation": "wǒ jiù shuō ma",
            "translation": "See, I said so!"
          },
          {
            "phrase": "你看，我不是早就说过了吗！",
            "pronunciation": "wnǐ kàn, wǒ bù shì zǎo jiù shuō guò le ma!",
            "translation": "See? Didn’t I already tell you!"
          }
        ]
      }
  ]
}
```