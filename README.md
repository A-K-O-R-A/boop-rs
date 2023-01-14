# Boop
This project is heavily inspired by the [Boop App](https://boop.okat.best/) for macOS by [Ivan Mathy](https://github.com/IvanMathy). But why should macOS users be the only ones to enjoy this wonderful piece of software? To make this wonderful app available to everyone I created a clone of the Boop App in Rust.

## Usage

Right now this project is focused mainly on the CLI part which allows you to quickly convert strings in the terminal like this:

```bash
$ curl -s https://dummy.restapiexample.com/api/v1/employee/1 | boop run json.format
{
  "status": "success",
  "data": {
    "id": 1,
    "employee_name": "Tiger Nixon",
    "employee_salary": 320800,
    "employee_age": 61,
    "profile_image": ""
  },
  "message": "Successfully! Record has been fetched."
}
```

Instead of piping the input to boop directly you can also use it like this

```bash
$ boop run base64.decode "SGVsbG8gV29ybGQh"
Hello World!
```

## Plugins
By default a few plugins directly embedded into the application are provided and always available but there is also an option to use external plugins. Right now only external plugins in javascript are supported. You can easily write your own plugins by just copying one of the examples in the `plugins` folder and changing the two functions.

The plugin just needs to export two functions `metadata(): Metadata` and `run(state: string): string` and are loaded at runtime, so it it is very similiar to the plugins used by the original boop app.

If you want to use your own plugins just provide the `plugins-folder` option like `-p, --plugins-folder <PLUGINS_FOLDER>`.

Other loaders than the javascript one (provided by the [`boa-engine`](https://github.com/boa-dev/boa)) might be added in the future(for example `lua`) but are currently out of scope for this project.

## GUI 
The original Boop App is focused on the GUI so originally I wanted to recreate that, unfortunatly writing a GUI application in rust without any prior knowledge turned out to be harder than I thought. So for now this project is manly focused on the CLI.

## Installation
Right now this project is not published anywhere so you'll need to clone this repository and build it yourself for now. More default plugins will be added and the CLI will receive a bit of polish before it's first proper release.

If you just want to try out the project you can do so using the `cargo install` command.
```bash
cargo install --git https://github.com/A-K-O-R-A/boop-rs/ boop
```