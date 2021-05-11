# How to create a RESTful API with Rust and Rocket on Windows

### **Introduction**

Rust language enables you to write faster and and more reliable code. It is staticky typed language which ensures type and memory safety if you choose to write to safe code. Rocket web framework for Rust lets you create web applications. According to Rocket framework such web applications are:

- Fast
- Secure
- Does not compromise type-safety, flexibility and usability

Keeping above advantages in mind, you will be creating a RESTful API in Rust via Rocket framework in this tutorial.

## **Pre-requisites**

- Rocket framework requires *Rust Nightly build*. You can install Rust using [rustup](https://rustup.rs/) as instructed on the site.

- It is recommended to have [Git for Windows](https://git-scm.com/downloads) installed and a [github](https://github.com/) account for source control.


## **Step 1: Configuring Rust Nightly**

There are two methods you can take to configure Rust Nightly version as your toolchain. First, setting it as your default toolchain by running the following command on the command prompt:

```rustup default nightly```
