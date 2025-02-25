# Elite Dangerous Rank Percentage Visualizer (Rust Rewrite)

![elite_screenie](https://github.com/user-attachments/assets/1424e453-dedc-4c3a-8c0b-70e94965a151)
.png)

## 🚀 Welcome, CMDR!
Welcome and o7! This is a **Rust-based rewrite** of the original project by [lilykmoto](https://github.com/lilykmoto), designed to provide a **clear, percentage-based visualization** of your Elite Dangerous ranks.

Elite Dangerous doesn’t show you rank progression percentages, but they **are** stored in your journal files. This program parses those files and provides a **detailed breakdown** of your progression!

---

## 📌 Table of Contents
- [Features](#features)
- [Installation](#installation)
  - [Linux Installation](#linux-installation)
  - [Windows Installation](#windows-installation)
- [Usage](#usage)
  - [Providing a Journal File](#providing-a-journal-file)
- [Contributing](#contributing)
- [Acknowledgments](#acknowledgments)
- [License](#license)

---

## 🎯 Features
✅ Parses Elite Dangerous journal files for rank progress values
✅ Displays **rank percentage progress** towards the next rank
✅ Shows **detailed progress bars** for each rank
✅ Works on **Windows, Linux, and Proton** installations
✅ Simple, lightweight, and fast execution

---

## 🔧 Installation

### 🐧 Linux Installation
Ensure you have **Rust** installed. If not, install it with:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then, clone the repository:
```sh
git clone https://github.com/your-username/original-project.git
cd original-project
```
To run the program without compiling:
```sh
cargo run
```
To build a release version:
```sh
cargo build --release
```

### 🖥 Windows Installation
1. Install Rust using [rustup](https://rustup.rs/).
2. Open **PowerShell** and run:
```powershell
git clone https://github.com/your-username/original-project.git
cd original-project
```
3. To run the program without compiling:
```powershell
cargo run
```
4. To build a release version:
```powershell
cargo build --release
```

---

## 🕹 Usage
### 📂 Providing a Journal File

The program will prompt you to **enter the path to your Elite Dangerous journal file**.

By default, these are stored at:
- **Windows**: `C:\Users\[USERNAME]\Saved Games\Frontier Developments\Elite Dangerous`
- **Linux (Proton)**: `/Path-to-SteamLibrary/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous`

To get that file path, you can either:
1. **Drag and drop the file** into the terminal window (click back into the window before pressing Enter!).
2. **Right-click the file**, select **“Copy as Path”**, then paste it into the console.

💡 *No need to remove quotation marks!* The program will handle that automatically.

---

## 🤝 Contributing
Pull requests and suggestions are welcome! Feel free to open an **issue** or **PR** to discuss improvements.

---

## 🎖 Acknowledgments
This project is a Rust rewrite of the original [Elite Dangerous Rank Percentage Visualizer](https://github.com/lilykmoto/original-project) by **CMDR lilykmoto**. Special thanks to her for the original concept and implementation! 🫡

---

## 📜 License
This project is licensed under the **MIT License**. See **LICENSE** for more details.

**o7, CMDR! Fly safe!** 🚀
