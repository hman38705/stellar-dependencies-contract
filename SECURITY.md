# Security

This is an MVP scaffold, not an audited funding application. It never executes scanned code or manages funds. Input validation, fixed upstream hosts, disabled redirects, body limits, content security headers and server-only token handling reduce common risks; they do not replace production review.

The optional on-chain contract does not manage or send funds. For the intended behavior, storage layout, trust assumptions, threat model and known risks with mitigations, see [`docs/DESIGN.md`](docs/DESIGN.md).

Before publishing, enable your repository's private vulnerability reporting and document the actual reporting channel. Do not open public issues containing secrets. Rotate any leaked token immediately. No maintainer contact address is invented in this template.