## todo list
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

---
- generate component tree at run time
- generate HTML templates at build time and populate them at run time

---
- conditional rendering
- listing
- conditional attributes
- conditional attribute values (class, style, etc.)
