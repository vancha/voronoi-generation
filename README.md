# voronoi-generation
a simple test to see if i can generate a voronoi pattern, badly written.

can create many frames that can be made into a video with:

ffmpeg -r 60 -f image2 -s 800x600 -i vorono%d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p test.mp4

