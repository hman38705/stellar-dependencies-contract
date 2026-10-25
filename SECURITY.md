# Security

This is an MVP scaffold, not an audited funding application. It never executes scanned code or manages funds. Input validation, fixed upstream hosts, disabled redirects, body limits, content security headers and server-only token handling reduce common risks; they do not replace production review.

The optional on-chain contract does not manage or send funds. For the intended behavior, storage layout, trust assumptions, threat model and known risks with mitigations, see [`docs/DESIGN.md`](docs/DESIGN.md).

## Reporting a vulnerability

Please report security vulnerabilities privately through GitHub's private vulnerability reporting form:

https://github.com/OWNER/REPO/security/advisories/new

Replace `OWNER/REPO` with the actual repository owner and name before publishing. Private vulnerability reporting must be enabled in the repository settings (Settings → Security → Private vulnerability reporting → Enable) for this form to be active.

Do not open public issues containing secrets or vulnerability details. If a token or other secret is leaked, rotate it immediately. No maintainer contact address is invented in this template.
