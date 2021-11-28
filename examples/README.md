# Welcome
This directory contains examples of how to use Zeiver.


## How To Run
To run an example type into the terminal `cargo run --example [NAME OF EXAMPLE FILE]`.

| File Name | Description of Example|
--- | --- |
| all.rs | Normal usage of Zeiver 1 directory deep. *(Includes/uses everything)* 
| no_recording.rs | Normal usage of Zeiver 1 directory deep. *(Uses everything except Recorder)* 
| scan.rs | Scans the OD. |
| record.rs | Only use the Recorder *(and Scraper)* to record file links in the OD. *(includes stats file)* |
| record_no_stats.rs | Only use the Recorder *(and Scraper)* to record file links in the OD. *(excludes stats file)* |
| headers.rs | Print the Response headers of all ODs |
| reject.rs | Use the Recorder (*and Scraper*) to record file links that are not `.php` or `.md`. | 




## Disclaimer

All ODs used here are open source demo ODs which can be found on GitHub.