# Zeiver
**Zeiver** is designed to *scrape* and *download* content recursively from static ODs
*(open directories)*. It also provides a means of *recording* links and *scanning* ODs for content.

__*Zeiver does not download the entire OD itself, only the files.__

For dynamic ODs *(JavaScript focused Open Directories)* use [Zyod!](https://github.com/ZimCodes/Zyod/)

For ease of use, check out the [Zeiver configurator](https://zimtools.vercel.app/zeiver).

![Build](https://github.com/ZimCodes/Zeiver/actions/workflows/build.yaml/badge.svg)

## Table of Contents
* [Features](#features)
   * [Normal Workflow](#normal-workflow)
   * [More](#more)
* [Open Directory Support](#open-directory-support)
* [Installation/Update](#installationupdate)
* [Sample](#sample)
* [Commands](#commands)
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
Supported ODs can be found in the [wiki page](https://github.com/ZimCodes/Zeiver/wiki/Supported-Open-Directories).

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

`zeiver -H "accept-language$fr-CH, en;q=0.8, de;q=0.7" -o "./Cool_Content" https://example.com/xms/imgs`

## Commands
Check out the [wiki page](https://github.com/ZimCodes/Zeiver/wiki/Commands) for a list of commands to use.

---
## Extra Info
### URL is too long?
Having trouble entering a long URL in the terminal? Place them inside an input file and use `--input-file` instead.

### Can't access an OD because of certificates?
Trying using the `--all-certs` option, but *be wary* with this option.

### Content exists, however Zeiver isn't scraping/recording/downloading/scouting any of them.
Some ODs will send Zeiver **HTML Documents** without any content *(links to files/folders)* from 
the OD.
This is because Zeiver retrieves an HTML Document **without JavaScript** & some ODs *will not work* without it.

[Use Zyod instead!](https://github.com/ZimCodes/Zyod/) 

---
## License 
Zeiver is licensed under the MIT and Apache 2.0 Licenses.

See the [MIT](https://github.com/ZimCodes/Zeiver/blob/main/LICENSE-MIT) and [Apache-2.0](https://github.com/ZimCodes/Zeiver/blob/main/LICENSE-APACHE_2.0) for more details.  

