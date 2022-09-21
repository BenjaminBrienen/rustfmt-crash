fn main() {
	{
		// 2 tabs of indentation is being interpreted as 100 spaces, causing rustfmt to think that the line is overflowing.
		// On the line below, 2 * `tab_spaces` + 26 = 110 > max_width, and error reporting is turned on, so it reports panicking in stdout. (26 is the width of the statement)
		// Why would someone set such a ridiculous value for `tabs_spaces`?
		// One reason might be to make it super obvious when the wrong indentation is being used if for some reason rustfmt fails to work properly in that respect.
		// rustfmt would panic on a more reasonable value in extremely nested scopes or longer statements as long as max_width is exceeded.
		
		// This should not be an error because `tabs_spaces` does not inform how many spaces a tab is rendered as (its width), especially since "hard tabs" aren't a fixed width.
		// It should only describe how many spaces to use for indentation when the indentation style is "soft".
		// Tabs should be treated as one character - or a configurable number of characters - for the purposes of checking against max_width.
		// `tab_spaces` should only be used in the process of overwriting the current line-leading indentation (including doc comments)
		
		// The whole point of using "hard tabs" is allowing the developer to configure their visual indentation size
		// preferences as an editor setting instead of enforcing a fixed width by using a set number of spaces in the source file.
		// Hard vs soft tabs are fundementally different, and should be treated as such by a formatter.
		
		// The needed fixes are:
		
		// Clarity: `tab_spaces` should be renamed to `soft_indentation_size`.
		// Clarity: `hard_tabs` should be renamed to `use_hard_indentation` or `intendation_style` (with options `"hard"` or `"soft"`).
		// Implementation: `soft_indentation_size` should only come into effect if `use_hard_indentation`/`intendation_style` is `true`/`"hard"`.
		// New: Create a config called `hard_indentation_size` to allow developers to enforce the current behavior.
		
		// In summary, rustfmt treats tabs as a fixed width described by `tabs_spaces` instead of 
		println!("Hello, world!");
	}
}
