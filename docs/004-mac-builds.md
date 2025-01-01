# Mac Builds

```sh
hdiutil verify kamekai_2.0.1_aarch64.dmg
```

```sh
spctl --assess -v kamekai_2.0.1_aarch64.dmg


kamekai_2.0.1_aarch64.dmg: rejected
source=no usable signature
```

The `codesign --force --sign - kamekai_${{ env.APP_VERSION }}_aarch64.dmg` results in
```sh
ls -l@ kamekai_2.0.1_aarch64.dmg 
-rw-r--r--@ 1  3000838 Jan  1  2025 kamekai_2.0.1_aarch64.dmg
	com.apple.quarantine	     21 

```

The quarantine is removed with
```sh
xattr -d -r com.apple.quarantine kamekai_2.0.1_aarch64.dmg
```

Local storage, caceh, etc is stored in
```sh
cd ~/Library/WebKit/com.kamekai.app/WebsiteData
```

Or in
```sh
cd ~/Library/Application Support/com.kamekai.app/
```

```sh
sudo codesign --force --deep --sign - /Applications/kamekai.app
```