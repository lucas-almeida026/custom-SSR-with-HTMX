## todo list
- What is the objective of this project?
### v1
- [x] watch multiple files
- [x] parse JSX into AST
- [ ] extract HTML and JS from AST
- [ ] handle runtime functions usage in AST (mark nodes that uses .get to implement client side logic later)

---
### todo sub
- [ ] traverse JSX element (basic parts)
- [ ] map event params (e.g. onClick) to event handlers from the stmt vector
	- errors [mismatched handler names, undefined handlers, top level async/await]
- [ ] build param list representation
	- errors [mismatched param names, undefined params, top level async/await]
- [ ] generate HTML template from JSX expression
	- [ ] traverse JSX expression
	- [ ] generate unique ids for elements that use client side logic

- identify condicional rendering
- identify list rendering
- identify nexted object props
---
### Decisions
- parse JSX into handlebars templates: this makes viable to use this tool alongside a wide range of languages that has packages for parsing handlebars
- start from javascript ecosystem because it is based on JSX anyway
