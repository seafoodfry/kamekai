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


Auth via the browser
https://dev.to/randomengy/using-auth0-with-tauri-14nl

A "Tauri app registered protocol" refers to a custom URL scheme that you can define within a Tauri application, allowing the app to be launched directly when a link using that protocol is clicked, essentially acting as a deep linking mechanism; for example, if you register "myapp" as your protocol, clicking a link like "myapp://somedata" would open your Tauri app and potentially pass along the "somedata" information

https://crates.io/crates/tauri-plugin-deep-link

We already had another plugin installed,
https://crates.io/crates/tauri-plugin-opener

Plugin permissions exercise
https://v2.tauri.app/learn/security/using-plugin-permissions/