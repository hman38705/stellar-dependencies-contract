# Security

This is an MVP scaffold, not an audited funding application. It never executes scanned code or manages funds. Input validation, fixed upstream hosts, disabled redirects, body limits, content security headers and server-only token handling reduce common risks; they do not replace production review.

Before publishing, enable your repository's private vulnerability reporting and document the actual reporting channel. Do not open public issues containing secrets. Rotate any leaked token immediately. No maintainer contact address is invented in this template.

## Related documents

- [`docs/AUDIT_CHECKLIST.md`](docs/AUDIT_CHECKLIST.md) — pre-audit readiness criteria (spec completeness, test coverage, static analysis and known limitations). Use this to determine when the contract is ready for review.
- [`docs/INTEGRATION_DESIGN.md`](docs/INTEGRATION_DESIGN.md) — integration design covering how a shortlist flows from the frontend to the registry, trust boundaries, who pays gas, and a comparison of options.