---
title: macOS
parent: Power User
---

<!-- prettier-ignore-start -->
# macOS
{: .no_toc }

<details open markdown="block">
  <summary>
    Table of contents
  </summary>
  {: .text-delta }
1. TOC
{:toc}
</details>

<!-- prettier-ignore-end -->

## Resources

---

https://github.com/ChrisTitusTech/macutil

System Settings

Apple Account

-   Sign in with Apple account
-   Sign-in & Security
    -   Setup recovery key (WDJ9-Y7CX-MMGL-BSPD-N6E3-648D-CWEC).
-   Payment & Shipping
    -   Add “Shipping Address”
-   iCloud
    -   Enable “Advanced Data Protection”
-   Enable “Contact Key Verification”

Network

-   Enable Firewall
    -   Turn on “Enable Stealth mode” under Options

Battery

-   Turn off “Slightly dim the display on battery”
-   Turn on “Prevent automatic sleeping on power adapter when the display is off”

General

-   About
    -   Storage Settings -> Empty Trash automatically
-   Software updater
    -   Automatic Updates -> Enable all the options
-   Storage
    -   Delete unnecessary apps (can also be found in Finder -> Applications)
-   AirDrop & Handoff
    -   Turn off “Airplay Receiver”
    -   Set password for Airplay
-   AutoFill & Passwords
    -   Turn on “Delete After Use” verification codes
-   Language & Region
    -   Temperature - Celsius
    -   Measurement system - Metric
    -   First day of week - Monday
    -   Date format - 2025-8-19

Accessibility

-   Zoom
    -   Enable “Use trackpad gesture to zoom”
-   Display
    -   Enable “Reduce transparency”
    -   Text size “20 pt”
    -   Disable “Shake mouse pointer to locate”

Appearance

-   Accent color “Green”
-   Sidebar icon size “Medium”
-   Show scroll bars “When scrolling”
-   Click in the scroll bar to “Jump to the spot that’s clicked”

Control Center

-   Bluetooth - Show in menu bar
-   Display - Always show in menu bar
-   Sound - Always show in menu bar
-   Battery - Show percentage
-   Keyboard brightness - Show in menu bar
-   Clock - Display the time with seconds
-   Spotlight - Don’t show in menu bar
-   Automatically hide and show the menu bar - Never

Desktop & Dock

-   Double click a window’s title bar to - fill
-   Automatically hide and show the dock - turn on

Display

-   Advanced
    -   Turn on “Automatically reconnect to any nearby Mac or iPad”

Screen saver

-   Shuffle Landscape - Continuously

Apple Intelligence & Siri

-   Disable

Spotlight

-   Turn off “Help Apple Improve Search”
-   Uncheck “Websites” in Search Results

Notifications

-   Show previews “Never”https://code.visualstudio.com/docs
-   Turn off all “Application notifications” except “Wallet”

Sound

-   Alert sound “Submerge”
-   Alert volume “Level 3”
-   Turn off “Play sound on startup”

Focus

-   Delete schedules from “Do Not Disturb”

Screen Time

-   Enable “Screen distance”

Lock Screen

-   Start screen saver when inactive - Never
-   Turn display off on battery when inactive - For 10 minutes
-   Turn display off on power adapter when inactive - For 10 minutes
-   Require password after screen saver begins or display is turned off - Immediately

Privacy & Security

-   Enable FileVault
-   Turn off “Apple Intelligence Report”

Game Center

-   Turn off

iCloud

-   Turn on Advanced Data Protection

Trackpad

-   Point and Click
    -   Tracking Speed - 5
    -   Click - Light
    -   Force Click and haptic feedback - Off
    -   Look up & data detectors - On
    -   Secondary click - Click or tap with two fingers
    -   Tap to click - On
-   More gestures
    -   Swipe between pages - Scroll left or right with two fingers
    -   Swipe between full-screen applications - Swipe left or right with fingers
    -   Notification Center - off
    -   Mission Control - Swipe up with three fingers
    -   Auto Expose - Off
    -   Launchpad - On (Pinch with thumb and three fingers)
    -   Show desktop - On (Spread with thumb and three fingers)

Keyboard

-   Key repeat rate - Max value
-   Turn keyboard backlight off after inactivity - After 1 minute
-   Press globe key to - Do nothing
-   Remove all text replacements under “Text Input”
-   Keyboard shortcuts
    -   Modifier Keys
        -   Caps Lock - Escape
        -   Globe key - Command

In terminal

-   Run “defaults write -g ApplePressAndHoldEnabled -bool false” to enable repeating keys on long press.
-   Run “defaults write com.apple.screencapture disable-shadow -bool true” to disable border when taking window screenshots

Open screenshot app

-   Options - Set save location to kushaj -> Pictures -> Screenshots

Open Finder

-   Settings (from top-left)
    -   General
        -   New Finder Windows show - kushaj
    -   Tags
        -   Hide all tags
    -   Sidebar
        -   Show recents, desktop downloads, kushaj, icloud, locations, applications
    -   Advanced
        -   Check “Show all filename extensions”
        -   Uncheck “Show warning before changing an extension”
-   Select “Show items as icons, …”

Remove unnecessary apps from the bottom dock. These are left

-   Finder

Set the view options on the desktop

Open App Store and goto accounts, and accept is prompted to setup your account.

Enable state manager

Download chrome

Download vscode

-   disable telemetry
-   move to applications folder
-   run the "add to path" script using Ctrl-Shift-P

Download homebrew

-   “xcode-select --install” to install xcode command line tools from terminal
-   Get latest “.pkg” from https://github.com/Homebrew/brew/releases
-   After installing add `eval "$(/opt/homebrew/bin/brew shellenv)"` to .zprofile

`defaults write com.apple.dock autohide-delay -float 0; defaults write com.apple.dock autohide-time-modifier -int 0;killall Dock` - to hide animatio of opening/closing dock.

https://apps.apple.com/us/app/snap/id418073146?mt=12 to use win + number key to open items from dock

Setup word

-   https://massgrave.dev/office_for_mac

Setup github ssh key

create `~/github` repo and put `frontend-docs` and `personal-backup` repo.

brew install scroll-reverser -> to set the scroll for trackpad and mouse independent of each other

brew install asdf
asdf plugin add ruby https://github.com/asdf-vm/asdf-ruby.git
brew install libyaml
add in `~/.zshrc` the path `export PATH="${ASDF_DATA_DIR:-$HOME/.asdf}/shims:$PATH"`
asdf install ruby 3.4.1
goto `frontend-docs` repo and run `bundle install`

Setup oh-my-zsh
`sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"`
setup powerlevel10k from github readme https://github.com/romkatv/powerlevel10k
setup zsh-syntax-highlighting plugin https://github.com/zsh-users/zsh-syntax-highlighting
setup zsh-autosuggestions plugin https://github.com/zsh-users/zsh-autosuggestions

Download rectangle app

Download "LM Studio"

Things to setup

-   Cleanshot X (NHQVHJZH-JLJXSDSK-RCQSXCLC-YVJQCPFP)
-   https://alt-tab-macos.netlify.app/
-   https://github.com/exelban/stats
