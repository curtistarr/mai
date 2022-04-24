#!/bin/sh

PROJECT=MAI

RESOURCES_DIR=./resources
ICON_DIR=$RESOURCES_DIR/$PROJECT.iconset
ORIGINAL_ICON=$RESOURCES_DIR/icon.png
ICON_FILE=$PROJECT.icns
ICON_PATH=$RESOURCES_DIR/$ICON_FILE

RELEASE_DIR=./target/release
BINARY_PATH=$RELEASE_DIR/$PROJECT
RELEASE_BUNDLE_DIR=$RELEASE_DIR/bundle
BUNDLE_APP_DIR=$RELEASE_BUNDLE_DIR/$PROJECT.app
BUNDLE_CONTENTS_DIR=$BUNDLE_APP_DIR/Contents
BUNDLE_INFO_PLIST_PATH=$BUNDLE_CONTENTS_DIR/Info.plist
BUNDLE_MACOS_DIR=$BUNDLE_CONTENTS_DIR/MacOS
BUNDLE_RESOURCES_DIR=$BUNDLE_CONTENTS_DIR/Resources

SHORT_VERSION=$(awk '/version/ {print substr($3, 2, length($3) - 2)}' Cargo.toml)
LONG_VERSION=$(date +"%y%m%d.%I%M%S")

echo Deleting old relase bundle at $RELEASE_BUNDLE_DIR
rm -rf $RELEASE_BUNDLE_DIR

echo Building release
cargo build --release

echo Creating $ICON_DIR
mkdir $ICON_DIR

echo Creating scaled icons
for SIZE in 16 32 64 128 256 512; do
  RETINA_SIZE=$(($SIZE * 2))
  sips -z $SIZE $SIZE $ORIGINAL_ICON -o $ICON_DIR/icon_${SIZE}x${SIZE}.png
  sips -z $RETINA_SIZE $RETINA_SIZE $ORIGINAL_ICON -o $ICON_DIR/icon_${SIZE}x${SIZE}x2.png
done

echo Creating $RESOURCES_DIR/$PROJECT.icns
iconutil -c icns -o $RESOURCES_DIR/$PROJECT.icns $ICON_DIR

echo Deleting $ICON_DIR
rm -rf $ICON_DIR

echo Creating bundle directories
mkdir -v $RELEASE_BUNDLE_DIR
mkdir -v $BUNDLE_APP_DIR
mkdir -v $BUNDLE_CONTENTS_DIR
mkdir -v $BUNDLE_MACOS_DIR
mkdir -v $BUNDLE_RESOURCES_DIR

echo Moving binary to relase bundle $BUNDLE_MACOS_DIR
mv $BINARY_PATH $BUNDLE_MACOS_DIR

echo Moving icons to release bundle $BUNDLE_RESOURCES_DIR
mv $ICON_PATH $BUNDLE_RESOURCES_DIR

echo Creating Info.plist $BUNDLE_INFO_PLIST_PATH
plutil -create xml1 $BUNDLE_INFO_PLIST_PATH
plutil -insert CFBundleDevelopmentRegion -string "English" $BUNDLE_INFO_PLIST_PATH
plutil -insert CFBundleDisplayName -string "$PROJECT" $BUNDLE_INFO_PLIST_PATH
plutil -insert CFBundleExecutable -string "$PROJECT" $BUNDLE_INFO_PLIST_PATH
plutil -insert CFBundleIconFile -string "$ICON_FILE" $BUNDLE_INFO_PLIST_PATH
plutil -insert CFBundleIdentifier -string "com.github.tarrcurtis.mai" $BUNDLE_INFO_PLIST_PATH
plutil -insert CFBundleInfoDictionaryVersion -string "6.0" $BUNDLE_INFO_PLIST_PATH
plutil -insert CFBundleName -string "$PROJECT" $BUNDLE_INFO_PLIST_PATH
plutil -insert CFBundlePackageType -string "APPL" $BUNDLE_INFO_PLIST_PATH
plutil -insert CFBundleShortVersionString -string "$SHORT_VERSION" $BUNDLE_INFO_PLIST_PATH
plutil -insert CFBundleVersion -string "$LONG_VERSION" $BUNDLE_INFO_PLIST_PATH
plutil -insert CSResourcesFileMapped -bool true $BUNDLE_INFO_PLIST_PATH
plutil -insert LSRequiresCarbon -bool true $BUNDLE_INFO_PLIST_PATH
plutil -insert NSHighResolutionCapable -bool true $BUNDLE_INFO_PLIST_PATH