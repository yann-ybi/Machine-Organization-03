My name is Yann Youbi. I work individually.

I have received help from the ed stem posts from the professor an example on how to use clap.
And I have used a large image posted by one of my TA on ed stem in order to test my code.

I do believe my code is not too badly implemented. I do think however it could've been done with less lines of codes. And it could be more efficient.

For this code, I made a function for each rotations: 
- 90 degree row major
- 90 degree column major
- 180 degree row major
- 180 degree column major

- 0 degree 
- 270 degree row major
- 270 degree column major 
- flip horizontal row major
- flip vertical row major
- flip horizontal column major
- flip vertical column major
- transpose row major
- transpose column major

Each function uses either my arr2 implemented row major or column major iterator followed by a map function calling a get function on the desired rotated coordinates to collect the elements of type T in their rotated order inside of a vector. Which is used to create an RgbImage struct and read to standard output.
I also use clap for my command lines arguments and match to control the flow of which function gets executed based on the command line arguments given.

Speeds Measurements using howth.ppm image provided by a TA on ed stem:

90 degree rotation
- row major:      0.325082176s	|	 predicted: 2
- column major: 373.052061481s  |	 predicted: 3

180 degree rotation
- row major:      0.453795135s	|        predicted: 1
- column major: 376.988145464s	|    	 predicted: 4

My prediction to that a 90 degree rotation with a column major iteration would take longer than the one with a row major iteration is accurate !
Despite having the same final locality, 90 degree column major is about 10 times slower than row major iteration
	- my arr2 iterator implementation of the  next function iterates by default in row major order on a from-row-major default constructor
	- There are 3 loops nested in my column major iterator to overcome that row major default set up compared to  only one used for my row major iterator

My prediction that 180 degree row major would be the fastest is  about 0.12 seconds inaccurate. With 90 degree row major being the fastest
	- This can be due to the fact that there are 4 additions to get each rotated coordinates using the formulas.
While a the 90 degree rotation formula only require 2 additions 
	- Also on my  90 degree row major implementation, I switch the original width and height of the image (width and height becomes the same as the it would become once rotated) before rotating the image using row major iteration  
So in terms of locality it has mostly hits for reading AND writing just like the 180 degree row major rotation

My prediction on having 180 degree rotation with column major iteration taking the most time compared to all the previous rotations mentioned is accurate.
	- bad locality, with all misses on reading and writing
	- 3 nested loops
	- 4 additions to get the rotated coordinates

It is really interesting to analyze programs speed and understand the differences of speeds from a locality perception without forgetting the arithmetics that can blur it.
My predictions were pretty accurate in overall, even though I was startled by the marge of difference which was bigger than expected.

		other speeds	
Rotate 270
Row major: 	0.28807s | 288.07ms
Column major: 	351.37s

Rotate 0:  	0.03343 | 33.430653ms

Flip
Row major:   0.229 seconds |  229.411228ms
Column major:

Transpose 
Row major:
Column major: 395.725911098s