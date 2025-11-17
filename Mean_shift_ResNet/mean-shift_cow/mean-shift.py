import time
import os
import random
import math
import torch
import numpy as np

# run `pip install scikit-image==0.18.3` to install skimage, if you haven't done so.
# If you use scikit-image>=0.19, you will need to replace the `multichannel=True` argument with `channel_axis=-1`
# for the `skimage.transform.rescale` function
from skimage import io, color
from skimage.transform import rescale

def distance(x, X):
    distances = X.clone()
    for p in range(len(X)):  # Fixed: was len(X)-1, should be len(X)
        distances[p] = torch.sqrt(torch.sum((X[p] - x) ** 2))
    return distances
        
def distance_batch(x, X):
    return torch.sqrt(torch.sum((X - x) ** 2, dim=1))

def gaussian(dist, bandwidth):
    gauss = dist.clone()
    for i, x in enumerate(dist):
        gauss[i] = np.exp(x ** 2 / (2 * bandwidth ** 2))
    return gauss

def update_point(weight, X):
    new_point = torch.zeros(X.shape[1])
    total_weight = 0
    
    for i in range(len(X)):
        new_point += weight[i] * X[i]
        total_weight += weight[i]
    
    return new_point / total_weight

def update_point_batch(weight, X):
    return torch.sum(weight[:, None] * X, dim=0) / torch.sum(weight)

def meanshift_step(X, bandwidth=2.5):
    X_ = X.clone()
    for i, x in enumerate(X):
        dist = distance(x, X)
        weight = gaussian(dist, bandwidth)
        X_[i] = update_point(weight, X)
    return X_

def meanshift_step_batch(X, bandwidth=2.5):
    N = X.shape[0]
    X_new = X.clone()

    for i in range(N):
        dist = distance_batch(X[i], X)
        weight = gaussian(dist, bandwidth)
        
        X_new[i] = update_point_batch(weight, X)

    return X_new

def meanshift(X):
    X = X.clone()
    for _ in range(20):
        X = meanshift_step(X)   # slow implementation
        #X = meanshift_step_batch(X)   # fast implementation
    return X

scale = 0.25    # downscale the image to run faster

# Load image and convert it to CIELAB space
image = rescale(io.imread('cow.jpg'), scale, multichannel=True)
image_lab = color.rgb2lab(image)
shape = image_lab.shape # record image shape
image_lab = image_lab.reshape([-1, 3])  # flatten the image

# Run your mean-shift algorithm
t = time.time()
X = meanshift(torch.from_numpy(image_lab)).detach().cpu().numpy()
# X = meanshift(torch.from_numpy(image_lab).to('mps')).detach().cpu().numpy()  # you can use GPU if you have one
t = time.time() - t
print ('Elapsed time for mean-shift: {}'.format(t))

# Load label colors and draw labels as an image
colors = np.load('colors.npz')['colors']
colors[colors > 1.0] = 1
colors[colors < 0.0] = 0

centroids, labels = np.unique((X / 4).round(), return_inverse=True, axis=0)

result_image = colors[labels].reshape(shape)
result_image = rescale(result_image, 1 / scale, order=0, multichannel=True)     # resize result image to original resolution
result_image = (result_image * 255).astype(np.uint8)
io.imsave('result.png', result_image)
