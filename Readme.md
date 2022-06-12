# tcracker

T\*\*\*\*\* app license checker crack. Very basic, does not patch UI related elements but haves the work done.

Finds the address where a pattern of bytes is located at and changes two instructions to modify the logic flow.

Execution might look like:

	alejandro@Mac ~ % tcracker /Applications/T*********.app
	Trying to crack ...
	    [*] Found address at: 0x82845
	    [*] 0x8284F: 85 -> 89
	    [*] 0x82851: 74 -> EB
	    [*] Done!

If already patched:

	alejandro@Mac ~ % tcracker /Applications/T*********.app
	Trying to crack ...
	    [*] Found address at: 0x7C9C6
	    [x] Error: Already patched

When failing to find the bytes pattern:

	alejandro@Mac ~ % tcracker /Applications/T*********.app
	Trying to crack ...
	    [x] Error: Failed to find pattern

If something went wrong (probably because selecting a similar pattern):

	alejandro@Mac ~ % tcracker /Applications/T*********.app
	Trying to crack ...
	    [*] Found address at: 0x7C9C6
	    [x] Error: Found unexpected bytes: 0x4B, 0x7A
