# How to create a RESTful API with Rust and Rocket on Windows

### **Introduction**

Rust language enables you to write faster and and more reliable code. It is staticky typed language which ensures type and memory safety if you choose to write to safe code. Rocket web framework for Rust lets you create web applications. According to Rocket framework such web applications are:

- Fast
- Secure
- Does not compromise type-safety, flexibility and usability

Keeping above advantages in mind, you will be creating a RESTful API in Rust via Rocket framework in this tutorial.

## **Pre-requisites**

- You should be familiar with Rust basics.

- Rocket framework requires *Rust Nightly build*. You can install Rust using [rustup](https://rustup.rs/) as instructed on the site.

- It is recommended to have [Git for Windows](https://git-scm.com/downloads) installed and a [github](https://github.com/) account for source control.

- For this tutorial, [Visual Studio Code](https://code.visualstudio.com/) is used as the code editor.

## **Step 1: Creating a project**

You can create a project folder manually and initialize it with cargo init command. Make sure you're in the created project directory and run the following command:

```cargo init --bin```

Alternatively, you can let the cargo tool create the project folder and initialize it using:

```cargo new rocket_quickstart --bin```

Note that we're instructing to _create a binary_ target when project compiles by specifying ```--bin``` flag.


## **Step 2: Configuring Rust Nightly**

There are two methods you can take to configure Rust Nightly version as your toolchain. First, setting it as your default toolchain by running the following command on the command prompt:

```rustup default nightly```

However, for this tutorial you may want to use the per-directory override once the project directory has been created. Make sure you're in the project directory and execute the command:

```rustup override set nightly```


Once Rust Nightly is configured, type ```code .``` from the project folder to open Visual Studio Code.

Here's a screenshot of the console:

![create-project](img/create_project.png)


## Step 3: **Setting up dependencies**