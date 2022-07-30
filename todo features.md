# Fundamental

## Autocompete

When typeing on User, Vin or License plate TextInput,
the best match will be shown in gray as a suggestion.
- Example: **[ Maria J|** uana **]** — enter key —> **[ Maria Juana| ]**
- Example: **[ Maria J|** uana **]** — enter key —> **[ J|** uana **Maria ]**

When writeing the User,
Vin(if it is complete) or License plate(if it is complete),
if there is a single match,
a sugestion of merging the two expedients will be shown.

## Similars Panel
On the expedient editor,
it will be posible to open a list
where the similar expedients will be shown.

It will be posible to merge the expedient of the list.

## Key mappings

- [x] New Tab: `Ctrl T`
- [x] Close Tab: `Ctrl W`
- [ ] Focus Tab: `Ctrl number`
- [ ] Focus Next Tab: `Ctrl Tab`
- [ ] Focus Previous Tab: `Ctrl Shift Tab`  
- [ ] Exit / Confirm: `F2`
- [ ] Exit / Cancel (reverting changes): `Esc`
- [x] Copy: `Ctrl C`
- [x] Paste: `Ctrl V`
- [x] Cut: `Ctrl X`
- [x] Focus Next InputText: `Tab`
- [x] Focus Previous InputText: `Shift Tab`
- [ ] Accept Autocomplete: `Enter`


## Keywords for match

Text to match: A, B, C, A B, B A C

| Filter | Matches          |
| ------ | ---------------- |
| A B    | A; B; A B; B A C |
| A_B    | A B              |
| A B +C | B A C            |
| +A B   | A B; B A C       |

---
# Additional

- Link pending -> done with Gaudi list
- Store backups on a diferent drive
- User panel/tab to change user on multiple expedients at once
- Similar panel with merge option




