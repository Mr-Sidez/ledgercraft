# The `.lcbooks` company file

A company file is UTF-8 JSON:

```json
{
  "format": "ledgercraft",
  "version": 1,
  "company": {
    "info": { "name": "Pine Ridge Woodworks", "fiscal_year_start_month": 1, "currency": "USD" },
    "accounts": [ { "id": 1, "number": "1000", "name": "Checking", "kind": "bank", "active": true } ],
    "customers": [ { "id": 34, "contact": { "name": "Dana Lin", "terms_days": 0 }, "active": true } ],
    "vendors": [],
    "items": [],
    "transactions": [
      {
        "id": 50, "date": "2026-01-12", "number": "1001", "memo": "", "voided": false,
        "body": {
          "type": "invoice", "customer": 34, "due_date": "2026-02-11", "ar_account": 3,
          "lines": [ { "item": null, "description": "Table", "qty": 1000, "rate": 240000, "account": 16 } ]
        }
      }
    ],
    "next_id": 60
  }
}
```

* Amounts (`rate`, `amount`, `debit`, `credit`) are integers in **cents**.
* Quantities (`qty`) are integers in **thousandths** (`1000` = 1).
* Dates are `YYYY-MM-DD`.
* Ids are unique across the whole file.
* `version` increases only on incompatible changes; LedgerCraft refuses files
  from a newer version rather than misreading them.
* Saves are atomic: the file is written to `NAME.lcbooks.tmp`, flushed, then
  renamed over the original.
