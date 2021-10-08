# Security Policy

## Reporting a Vulnerability

If you find a vulnerability, you should report it immediately. There are two
available courses of action, depending on the type of vulnerability.

These are classified using a severity scale, where the appropriate course of
action is as follows:
- **1**: This type of vulnerability is insignificant enough to leave as an issue
  in the repository using the provided "Security Report" template.
- **2**: This type of vulnerability is moderately serious and enough to warrant
  filing a report privately to `security@k-cs.co`
- **3**: This type of vulnerability is of the utmost severity and should be
  filed directly to our team via one of our support channels (DO NOT post
  publicly, message one of our available engineers instead).

### "Application Availability" [2]

This type of vulnerability is categorized as anything that could result in
a denial of service attack or similar. For example, the regular expression
vulnerability in `yargs` that made it possible to crash an application by
inputting text that triggered the recursive regular expression functionality
with an exponential time complexity for long enough to stall the program.

### "User Safety" [3]

This type of vulnerability is categorized as anything that could result in the
unlawful obtaining or exploitation of data from any end users. For example, an
SQLi vulnerability that would enable you to query a user database.

### Others [?]

For other kinds of vulnerability that are not explicitly mentioned here yet,
use common sense to determine its severity and follow the appropriate course of
action. If in doubt, follow the procedures for security level 2, as it is
a balance between secure reporting and alarm created.
