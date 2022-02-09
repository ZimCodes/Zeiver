# Zeiver
**Zeiver** is designed to *scrape* and *download* content recursively from static ODs
*(open directories)*. It also provides a means of *recording* links and *scanning* ODs for content.

__*Zeiver does not download the entire OD itself, only the files.__

For dynamic ODs *(JavaScript focused Open Directories)* use [Zyod!](https://github.com/ZimCodes/Zyod/)

For ease of use, check out the [Zeiver configurator](https://zimtools.xyz/zeiver).

![Build](https://github.com/ZimCodes/Zeiver/actions/workflows/build.yaml/badge.svg)

## Table of Contents
* [Features](#features)
   * [Normal Workflow](#normal-workflow)
   * [More](#more)
* [Open Directory Support](#open-directory-support)
* [Installation/Update](#installationupdate)
* [Sample](#sample)
* [Commands](#commands)
    * [Positional](#positional) 
    * [Options](#options)
        * [General](#general)
        * [Download](#download)
        * [Recorder](#recorder)
        * [File/Directory](#filedirectory)
        * [Grabber](#grabber)
* [Extra Info](#extra-info)
* [License](#license)

## Features
Zeiver currently has 4 major modules:
* Grabber *(HTTP)*
  * Grabs content from the internet. *(Webpage,files,etc)*
* Scraper
    * Recursively grabs all links from an OD.
* Downloader
    * Downloads content retrieved from Scraper (_or from a file_)
* Recorder
    * Saves a record of all files that were found in the OD
    * Records are saved to a file called *URL_Records.txt*. Name can be 
      changed using `--output-record`
    * Creates stat files *(JSON files containing statistical data about what was retrieved)*

***All components can be used independently.***

### Normal Workflow
The **Grabber** module repeatedly grabs a webpage for the Scraper to parse *(based on parameters)*.
The **Scraper** identifies the type of scrape method to use from the webpage and recursively scrapes the links from them.
Afterwards, the links are either sent to the
**Recorder** (_Disabled by default_), specified with:
* `--record-only`
* `--record`
 
*AND/OR*
 
**Downloader** (_Enabled by default_). The **Downloader** uses the **Grabber** to download
the files' data from the internet. The **Downloader** then writes the data to a newly created files.

### More
1. Uses asynchronous runtime.
2. Random & fixed delays of HTTP requests.
3. Ability to customize how files retrieved or not.
4. Scans an OD for content while transparently displaying the traversal process.
5. Cross-platform support

## Open Directory Support
Supported ODs can be found in [OD.md](https://github.com/ZimCodes/Zeiver/blob/main/OD.md).

## Installation/Update
To **install/update** Zeiver follow these steps:

1. Install Rust. 
    * If Rust is not installed, please follow the instructions [here](https://www.rust-lang.org/tools/install)
    
2. Once Rust is installed, open a CLI & type `cargo install --branch main --git https://github.com/ZimCodes/Zeiver`
    
    * _This will install Zeiver from Github_
    
3. And that's it! To use Zeiver, start each command with `zeiver`.

*To remove Zeiver, open a CLI & type `cargo uninstall zeiver`. 

## Sample
The following code downloads files from _example.com/xms/imgs_, saves them in a local directory called _Cool_Content_,
& sends a request with the ACCEPT-LANGUAGE header.

`zeiver -h "accept-language$fr-CH, en;q=0.8, de;q=0.7" -o "./Cool_Content" example.com/xms/imgs`

## Commands 
### Positional
__URL*s*...__

Link(*s*) to the OD(*s*) you would like to download content from. 
*_This is not needed if you are using `-i, --input-file`._

---
### Options
#### General
***-h, --help***

Prints help information.

***-U, --update***

Update to the *latest* version.

***-V, --version***

Prints version information

***-v, --verbose***

Enable verbose output

***--test***

Run a scrape test without downloading or recording. **Deactivates Downloader & Recorder**.

***--scan***

Scan ODs

Scan ODs displaying their content to the terminal. *A shortcut to enabling*
`--verbose` *&* `--test`. **Deactivates Downloader & Recorder**.

---
#### Download

***-d, --depth***

Specify the maximum depth for recursive scraping. 

How far to look into a directory(ies) to retrieve files. Can also be used to traverse subpages 
*(ODs with **previous** & **next** buttons)*. _Default: `20`_. **Depth of`1` is current directory.**

***-A, --accept***

Files to accept for scraping, downloading, & recording. *(Regex)*

Using Regex, specify which files to accept for scraping, recording, & downloading. Only the files 
that matches the regex will be acceptable for scraping, downloading, & recording.
_*This option takes precedence over `--reject, -R`*_.

Ex: `zeiver -A "(mov|mp3|lunchbox_pic1\.jpg|(pic_of_me.gif))"`

***-R, --reject***

Files to reject for scraping, downloading, & recording. *(Regex)*

Using Regex, specify which files to reject for scraping, downloading, & recording. Only the files 
that match the regex will be rejected for scraping, downloading, & recording. *_`--accept, -A` 
takes precedence over this option_*.

Ex: `zeiver -R "(jpg|png|3gp|(pic_of_me.gif))"`

---
#### Recorder

***--record***

Activates the Recorder

Enables the Recorder which saves the scraped links to a file. *Disabled by default.*
*_Option cannot be used with `--record-only`_.

***--record-only***

Save the links only

After scraping, instead of downloading the files, save the links to them. *_The downloader will be disabled when
this option is active. Enables Recorder instead._

***--output-record***

Name of record file

Changes the name of the record file. This file is where the recorder will store the links. 
*Default: `URL_Records.txt`*

***--no-stats***

Prevents Recorder from creating `_stat_` files.

The Recorder will no longer create `_stat_` files when saving scraped links to a file. *Default: `false`*
Ex: `stat_URL_Record.txt`

***--no-stats-list***

Prevent Recorder from writing file names to stat files

Stat files includes the names of all files in alphabetical order
alongside the number of file extensions. This option prevents the Recorder from adding file names
to stat files.

---
#### File/Directory

***-i, --input-file***

Read URLs from a local or external file

Read URLs from a file to be sent to the Scraper. *Each line represents a URL to an OD.

Ex: `zeiver -i "./dir/urls.txt"`

***--input-record***
Read URLs from a file containing file paths and create a stats file.

Read URLs from an input file which contains links to other files and create a stats file based on the results. This option is
for those who have a file filled with random unorganized links to a bunch of other files and want to take advantage of Zeiver's
*Recorder* module.

*Each line represents a URL to a file. **Activates Recorder**. Valid with `--verbose`,
`--output`, `--output-record`, `--no-stats-list`.

***-o, --output***

Save Directory.

The local directory path to save files. **Files saved by the *Recorder* are also stored here.**
_Default: `./`_

Ex: `zeiver -o "./downloads/images/dir"`

***-c,--cuts***

Ignores a specified number of remote directories from being created.
*_Only available when downloading. Default: `0`_

Ex: URL: `example.org/pub/xempcs/other/pics`

Original Save Location: `./pub/xempcs/other/pics`

`zeiver --cuts 2 www.example.org/pub/xempcs/other/pics` 

New Save Location: `./other/pics`

***--no-dirs***

Do not create a hierarchy of directories structured the same as the URL the file came from. 
All files will be saved to the current output directory instead. *_Only available when downloading._

---
#### Grabber
***--print-headers***

Prints all Response Headers to the terminal

Prints all available Response headers received from each url to the terminal.
**Option takes precedence over all other options**!

***--print-header***

Prints a Response Header to terminal

Prints a specified Response Header to the terminal for each url. **This Option takes precedence over all
other options**.

***--print-pages***

Prints the HTML Document to the terminal

Prints the HTML Document of each URL to the terminal for viewing. Allows you to see in the eyes 
of Zeiver. **This option takes precedence over all other options**.

***--https-only***

Use HTTPS only

Restrict Zeiver to send all requests through HTTPS connections only.

***-H, --headers***

Sets the default headers to use for every request. *_Must use the __'header$value'__ format. Each header must also be
**separated by a comma**._ 

Ex: `zeiver -H content-length$128,"accept$ text/html, application/xhtml+xml, image/webp"`

***-u***

The User Agent header to use. _Default: `Zeiver/VERSION`_

***-t, --tries***

The amount of times to retry a failed connection/request. _Default: `20`_
 
***-w, --wait***

Wait between each HTTP request for scraping.

Wait a specified number of *seconds* before sending each scraping request.

***--wait-download***

Wait between each HTTP request for download.

Wait a specified number of seconds before sending each download request.

***--retry-wait***

The wait time between each failed request. 

Whenever a request fails, Zeiver will wait the specified number of seconds before retrying again.
_Default: `10`_

***--random-wait***

Wait a random amount of seconds before each HTTP request for scraping.

Randomly waits a specified number of seconds before each scraping request. The time between 
requests will vary between 0.5 * `--wait,-w` *(inclusive)* to 1.5 * `--wait,-w` *(exclusive)*

***--random-download***

Wait a random amount of seconds before each HTTP request for download.

Randomly waits a specified number of seconds before each download request. The time between 
requests will vary between 0.5 * `--wait-download` *(inclusive)* to 1.5 * `--wait-download`
*(exclusive)*

***-T, --timeout***

Adds a request timeout for a specified number of seconds. **`0` means to never timeout.** 
_Default: `40`_

***-r, --redirects***

Maximum redirects to follow. _Default: `10`_

***--proxy***

The proxy to use.

Ex: `zeiver --proxy "socks5://192.168.1.1:9000"`

***--proxy-auth***

The basic authentication needed to use the proxy. *_Must use the __'username:password'__ format._

***--all-certs***

Accepts all certificates *(Beware!)*

Accepts all certificates even invalid ones. **Use this option at your own risk!**

---
## Extra Info
### URL is too long
Having trouble entering a long URL in the terminal? Place them inside an input file and use `--input-file` instead.

### Can't access an OD because of certificates
Trying using the `--all-certs` option, but *be wary* with this option.

### Content from OD exists, however Zeiver isn't scraping/recording/downloading/scouting any of them
Some ODs will send Zeiver **HTML Documents** without any content *(files/folders links)* from the OD.
This is because Zeiver retrieves an HTML Document **without JavaScript** & some ODs *will not work* without it.

---
## License 
Zeiver is licensed under the MIT and Apache 2.0 Licenses.

See the [MIT](https://github.com/ZimCodes/Zeiver/blob/main/LICENSE-MIT) and [Apache-2.0](https://github.com/ZimCodes/Zeiver/blob/main/LICENSE-APACHE_2.0) for more details.  

