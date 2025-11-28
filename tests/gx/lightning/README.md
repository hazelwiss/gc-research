Contains various tests for lightning. Spotlights, diffuse and specular lightning. The test are interactable through the controller. Even without controller support these tests are useable.

# Diffuse test

This tests adds a single light in a scene with a coloured sphere. The light and sphere can both be rotated. The point is that the light (symbolized by a yellow sphere) is meant to illuminate the part
of the sphere that it is facing, and the rest of the sphere should be dimly lit or completely dark.

### Controls:
- A:        switch between clamped and signed diffuse
- stick:    adjust light rotation
- substick: adjust sphere rotation

# Spotlight test

This tests spotlight lightning without diffuse/specular lights. The test creates a surface with 4 coloured quads and 4 coloured lights with the colours white, red, blue and green respectively.
The lights rotate in a circular motion showing the different effects of lights on differently coloured surfaces. The lights are denoted by 4 smaller quads in their respective colours.

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
