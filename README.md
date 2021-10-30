# Zeiver
**Zeiver** is designed to *scrape* and *download* content recursively from ODs _(open directories)_.
It also provides a means of *recording* links and *scanning* ODs for content.

__*Zeiver does not download the entire OD itself, only the files.__

For ease of use, check out the [Zeiver configurator](https://zimtools.xyz/zeiver).

## Table of Contents
* [Features](https://github.com/ZimCodes/Zeiver#features)
   * [Normal Workflow](https://github.com/ZimCodes/Zeiver#normal-workflow)
   * [More](https://github.com/ZimCodes/Zeiver#more)
* [Open Directory Support](https://github.com/ZimCodes/Zeiver#open-directory-support)
* [Installation](https://github.com/ZimCodes/Zeiver#installation)
* [Sample](https://github.com/ZimCodes/Zeiver#sample)
* [Commands](https://github.com/ZimCodes/Zeiver#commands)
    * [Positional](https://github.com/ZimCodes/Zeiver#positional) 
    * [Options](https://github.com/ZimCodes/Zeiver#options)
        * [General](https://github.com/ZimCodes/Zeiver#general)
        * [Download](https://github.com/ZimCodes/Zeiver#download)
        * [Recorder](https://github.com/ZimCodes/Zeiver#recorder)
        * [File/Directory](https://github.com/ZimCodes/Zeiver#filedirectory)
        * [Grabber](https://github.com/ZimCodes/Zeiver#grabber)
* [Extra Info](https://github.com/ZimCodes/Zeiver#extra-info)
* [License](https://github.com/ZimCodes/Zeiver#license)

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
the files' data from the internet. the **Downloader** then writes the data to a newly created files.

### More
1. Uses asynchronous runtime.
2. Random & fixed delays of HTTP requests.
3. Ability to customize how files retrieved or not.
4. Scans an OD for content while transparently displaying the traversal process.

## Open Directory Support
Supported ODs can be found in [OD.md](https://github.com/ZimCodes/Zeiver/blob/main/OD.md).

## Installation/Update
To **install/update** Zeiver follow these steps:

1. Install Rust. 
    * If Rust is not installed, please follow the instructions [here](https://www.rust-lang.org/tools/install)
    
2. Once Rust is installed, open a CLI & type `cargo install --branch main --git https://github.com/ZimCodes/Zeiver`
    
    * _This will install Zeiver from Github_
    
3. And that's it! To use Zeiver, start each command with `zeiver`.

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

Specify the maximum depth for recursive scraping. Can also be used to traverse subpages *(ODs with **previous** & **next** buttons)*.
_Default: `20`_. **Depth of`1` is current directory.**

***-A, --accept***

Files to accept for scraping

Using Regex, specify which files to accept for scraping. Only the files that matches the regex will be
acceptable for download. _*This option takes precedence over `--reject, -r`_.

Ex: `zeiver -A "(mov|mp3|lunchbox_pic1\.jpg|(pic_of_me.gif))"`

***-R, --reject***

Files to reject for scraping

Using Regex, specify which files to reject for scraping. Only the files that match the regex will be
rejected for download. *_`--accept, -a` takes precedence over this option_.

Ex: `zeiver -R "(jpg|png|3gp|(pic_of_me.gif))"`

---
#### Recorder

***--record***

Activates the Recorder

Enables the Recorder which saves the scraped links to a file.
*_Option cannot be used with `--record-only`_.

***--record-only***

Save the links only

After scraping, instead of downloading the files, save the links to them. *_The downloader will be disabled when
this option is active. Enables Recorder instead._

***--output-record***

Changes the name of the record file. This file is where the recorder will store the links. *Default: `URL_Records.txt`*

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

Read URLs from a file to be sent to the Scraper. *Each line represents a URL to an OD.

Ex: `zeiver -i "./dir/urls.txt"`

***--input-record***

Read URLs from an input file which contains links to other files and create a stats file based on the results. This option is
for those who have a file filled with random unorganized links to a bunch of other files and want to take advantage of Zeiver's
*Recorder* module.
*Each line represents a URL to a file. **Activates Recorder**. Valid with `--verbose`,
`--output`, `--output-record`

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
All files will be saved to the current output directory instead. 

*_Only available when downloading._

---
#### Grabber
***--print-headers***

Prints all Response Headers to the terminal

Prints all available Response headers received from each Request to the terminal.
**Option takes precedence over all other options**!

***--print-header***

Prints a Response Header to terminal

Prints a specified Response Header to the terminal for each url. **This Option takes precedence over all
other options**.

***--https-only***

Use HTTPS only

Restrict Zeiver to send all requests through HTTPS connections only.

***-H, --headers***

Sets the default headers to use for every request. *_Must use the __'header$value'__ format. Each header must also be
**separated by a comma**._ 

Ex: `zeiver -H content-length$128,"accept$ text/html, application/xhtml+xml, image/webp"`

***-U***

The User Agent header to use. _Default: `Zeiver/VERSION`_

***-t, --tries***

The amount of times to retry a failed connection/request. _Default: `20`_
 
***-w, --wait***

Wait a specified number of seconds between each scraping & download requests.

***--retry-wait***

The wait time between each failed request. _Default: `10`_

***--random-wait***

Wait a random amount of seconds between each request.

The time between requests will vary between 0.5 * `--wait,-w` (_inclusive_) to 1.5 * `--wait,-w` (_exclusive_)

***-T, --timeout***

Adds a request timeout for a specified number of seconds.

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

