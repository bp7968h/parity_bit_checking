#Program to Check the Parity Bit
Parity checks count the number of 1s within a bitstream. These store a bit that indicates whether the count was even or odd.

##Background
Parity bits are traditionally used for error detection within noisy communication
systems, such as transmitting data over analog systems such as radio waves. For example, the ASCII encoding of text has a particular property that makes it quite conveient for this scheme. Its 128 characters only require 7 bits of storage (128 = 27). That leaves 1 spare bit in every byte.
