# Contributing to Sui Rust Blockchain Course (201-JVM)

## 📌 Overview
Thank you for considering contributing to the **Sui Rust Blockchain Course**! This guide outlines the process for contributing to course materials, submitting exercises, and improving documentation.

---

## ** How to Contribute**
### **Fork the Repository**
1. Click the **Fork** button on GitHub.
2. Clone your forked repository:
   ```sh
   git clone https://github.com/YOUR-USERNAME/201-jvm.git
   cd 201-jvm
   ```
3. Create a new branch:
   ```sh
   git checkout -b feature-branch
   ```

### **Make Your Changes**
- For **bug fixes**, modify the necessary files and commit.
- For **new features**, add relevant documentation and tests.
- Follow **coding guidelines**:
  - Use **Rust formatting** (`cargo fmt`)
  - Follow **Move best practices**
  - Ensure **Axum/Yew compatibility** for Rust APIs and frontends

### **Submit a Pull Request (PR)**
1. Add your changes:
   ```sh
   git add .
   git commit -m "Added feature XYZ"
   git push origin feature-branch
   ```
2. Open a Pull Request on GitHub and request a review.

---

## ** Code Contribution Guidelines**
### **Rust Code Formatting**
- Ensure all Rust files are formatted with:
  ```sh
  cargo fmt
  ```
- Run Clippy for linting:
  ```sh
  cargo clippy -- -D warnings
  ```

### **Move Smart Contracts**
- Use **Sui’s best practices** for Move contracts.
- Validate contracts with:
  ```sh
  sui move build
  ```
- Run contract tests:
  ```sh
  sui move test
  ```

---

## ** Reporting Issues & Requesting Features**
- Use the **GitHub Issues** section to report **bugs, typos, or missing content**.
- Use the **Feature Request Template** in `.github/ISSUE_TEMPLATE/feature_request.md`.

---

## 🚀 **Happy Contributing!**
Your contributions make this course better for everyone! Feel free to reach out via GitHub discussions. 🎉
