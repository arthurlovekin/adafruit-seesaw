# Joy FeatherWing Overview: https://learn.adafruit.com/joy-featherwing/overview

Make a game or robotic controller with this Joy-ful FeatherWing.This FeatherWing has a 2-axis joystick and 5 momentary buttons (4 large and 1 small) so you can turn your feather board into a tiny game controller. This wing communicates with your host microcontroller over I2C so it's easy to use and doesn't take up any of your precious analog or digital pins. There is also an optional interrupt pin that can alert your feather when a button has been pressed or released to free up processor time for other tasks.
gaming_3632_iso_ORIG.jpg

This FeatherWing features Adafruit Seesaw technology - a custom programmed little helper microcontroller that takes the two analog inputs from the joystick, and 5 button inputs, and converts it into a pretty I2C interface. This I2C interface means you don't 'lose' any GPIO or analog inputs when using this 'Wing, and it works with any and all Feathers! You can easily stack this with any other FeatherWing because I2C is a shared bus. If you have an I2C address conflict, or you want to connect more than one of these to a Feather, there are two address-select jumpers so you have 4 options of I2C addresses
gaming_3632_quarter_ORIG.jpg

There's an optional IRQ (interrupt) line that you can use if you'd like the Wing to let you know when a button has been pressed. Since its optional (and most Feathers are perfectly happy polling the Wing for data) we left a bunch of solder jumpers so you can select what pin has the IRQ on it. Or just leave it disconnected!

Because multiple featherwings of all kinds can be stacked, the I2C address can be changed if necessary. The Joy Featherwing's default I2C address is 0x49, but soldering jumpers A0 and/or A1 can change this:

| A0    | A1    | Addr         |
|-------|-------|--------------|
| open  | open  | 0x49 (default) |
| closed| open  | 0x4A         |
| open  | closed| 0x4B         |
| closed| closed| 0x4C         |
