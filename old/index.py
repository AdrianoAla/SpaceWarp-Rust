from PIL import Image

def split_image(image_path, chunk_size):
    # Open the image
    try:
        image = Image.open(image_path)
    except IOError:
        print("Unable to open image file.")
        return
    
    # Get the width and height of the image
    width, height = image.size
    
    # Calculate the number of chunks in each dimension
    num_chunks_x = width // chunk_size
    num_chunks_y = height // chunk_size
    
    # Iterate over each chunk
    for y in range(num_chunks_y):
        for x in range(num_chunks_x):
            # Calculate the coordinates of the chunk
            left = x * chunk_size
            upper = y * chunk_size
            right = left + chunk_size
            lower = upper + chunk_size
            
            # Crop the chunk from the original image
            chunk = image.crop((left, upper, right, lower))
            
            # Save the chunk as a separate image
            chunk_filename = f"{x}_{y}.png"
            chunk.save(chunk_filename)
            print(f"Saved {chunk_filename}")

# Example usage
image_path = "./spacewarp.png"  # Replace with the path to your own image
chunk_size = 8

split_image(image_path, chunk_size)
