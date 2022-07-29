# Fundamental

## Autocompete

When typeing on User, Vin or License plate TextInput,
the best match will be shown in gray as a suggestion.
- Example:
<span style="white-space: normal">
	<span style="background: #333; padding: 4px 8px 4px 8px; color: #fff">
		Maria J|<span style="color: #999">uana</span>
	</span>
	-- enter key -->
	<span style="background: #333; padding: 4px 8px 4px 8px; color: #fff">
		Maria Juana|
	</span>
</span>

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

| Filter      | Matches                             |
| ----------- | ----------------------------------- |
| Maria Rosa  | Maria; Rosa; Maria Rosa; Rosa Maria |
| Maria_Rosa  | Maria Rosa                          |
| Maria +Rosa | Rosa; Maria Rosa; Rosa Maria        |
| +Maria Rosa | Maria Rosa; Rosa Maria              |


---
# Additional

- Link pending -> done with Gaudi list
- Store backups on a diferent drive
- User panel/tab to change user on multiple expedients at once
- Similar panel with merge option