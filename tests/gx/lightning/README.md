Contains various tests for lightning. Spotlights, diffuse and specular lightning. The test are interactable through the controller.

Even without controller support these tests may be used, and if no input is provided these tests will run on their own (at least until input is provided) and display some kind of output regardless.

# Ambient test

This is the simplest test, simply testing ambient lightning on a sphere. The sphere is put in a dark scene with minimal ambience. The sphere can be rotated.
Before input is provided the ambience will increase by itself.

### Controls:
- stick:    adjust light intensity
- substick: adjust sphere rotation

# Specular test

This tests adds a single light in a scene with a coloured sphere. The sphere can both be rotated. The light has an associated direction from which it would shine the brightest on the sphere.
The direction of the light is symbolized by the yellow arrow. 
Before input is provided the direction of the light will change itself automatically.

### Controls:
- stick:    adjust light direction
- substick: adjust sphere rotation

# Diffuse test

This tests adds a single light in a scene with a coloured sphere. The light and sphere can both be rotated. The point is that the light (symbolized by a yellow sphere) is meant to illuminate the part
of the sphere that it is facing, and the rest of the sphere should be dimly lit or completely dark.
Before input is provided the position of the light will change itself automatically.

### Controls:
- A:        switch between clamped and signed diffuse
- stick:    adjust light rotation
- substick: adjust sphere rotation

# Spotlight test

This tests spotlight lightning without diffuse/specular lights. The test creates a surface with 4 coloured quads and 4 coloured lights with the colours white, red, blue and green respectively.
The lights rotate in a circular motion showing the different effects of lights on differently coloured surfaces. The lights are denoted by 4 smaller quads in their respective colours.
Before input the lights will rotate automically around the plane.

### Controls:
- A:                   pauses/resumes light rotation
- DPAD up/down         adjust lights rotation speed
- stick right/left:    adjust lights brightness
- stick up/down:       adjust lights offset
- substick right/left: adjust lights range
- substick up/down:    adjust lights spread

# Credits

arrow by Alihan on Sketchfab: https://skfb.ly/ozHGW
sphere by thequinneffect on Sketchfab: https://skfb.ly/6XEn7
