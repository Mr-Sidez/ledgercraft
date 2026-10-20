# Clean-room policy

LedgerCraft re-creates the *workflow* of desktop small-business accounting.
Workflows, accounting rules and feature ideas are not protected by copyright;
specific code, artwork, text and file formats can be. These rules keep the
project legally clean. They follow the approach used by the ArtCraft Crafting
Apps (clean-room Rust re-implementations of Word, Excel, Photoshop and others).

This is project policy, not legal advice. Before a 1.0 release or any
commercial distribution, have the project reviewed by a lawyer.

## You may

* Use QuickBooks (or any accounting product) you are licensed to use as a black
  box: enter synthetic data, look at what it does, measure behaviour, and write
  notes in your own words.
* Read public documentation, help articles, accounting textbooks, GAAP/IFRS
  material, tax-authority publications and open specifications (OFX).
* Use generic accounting and feature names: Invoice, Bill, Pay Bills, Write
  Checks, Undeposited Funds, Chart of Accounts, Profit & Loss, A/R Aging.
* Read openly documented import formats (CSV, OFX/QFX/QBO, IIF) to let people
  bring their own data in.

## You may not

* Decompile, disassemble, debug, patch or read the program files of any Intuit
  product, or anything inside its installation folder.
* Reverse-engineer the proprietary `.QBW`, `.QBB`, `.QBM` or `.TLG` formats, or
  use code that does.
* Violate any licence agreement (EULA) you have accepted, including clauses
  against reverse engineering.
* Copy icons, logos, colour schemes, screenshots, artwork, help text, report
  templates, invoice templates, sample companies (including their names and
  data), wording of the default chart of accounts, or marketing copy.
* Use "QuickBooks", "Intuit" or their logos in LedgerCraft's name, icon,
  package names or in a way that suggests endorsement. Describing
  compatibility in plain text ("works like the QuickBooks Desktop workflow")
  with the trademark notice is fine.
* Copy code from GPL, AGPL or LGPL projects (GnuCash, KMyMoney, LedgerSMB…)
  or from any source whose licence is not compatible with MIT/Apache-2.0.
* Commit screenshots of other vendors' software or files produced by it.

## If you have worked at Intuit or seen its source code

Tell the maintainers before contributing. You may still help with
documentation, testing and areas unrelated to what you saw.
