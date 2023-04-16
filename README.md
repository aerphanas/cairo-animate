# cairo animate

the idea is generate 99 farme and merge it into 1 video using ffmpeg

```sh
ffmpeg -framerate 30 -i frame%d.png -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p output.mp4
```
<video width="320" height="240" controls>
  <source src="https://raw.githubusercontent.com/aerphanas/cairo-animate/main/output.mp4" type="video/mp4">
</video>
