import {
  Meme,
  checkResources,
  checkResourcesInBackground,
  crop,
  flipHorizontal,
  flipVertical,
  getMeme,
  getMemeKeys,
  getMemes,
  getVersion,
  gifChangeDuration,
  gifMerge,
  gifReverse,
  gifSplit,
  grayscale,
  inspect,
  invert,
  MemeSortBy,
  MemeStatisticsType,
  mergeHorizontal,
  mergeVertical,
  renderMemeList,
  renderMemeStatistics,
  resize,
  rotate,
  searchMemes,
} from "./js-binding.js";

export { Meme, getMeme, getMemes, getMemeKeys, searchMemes, getVersion };

export const Resources = {
  checkResources,
  checkResourcesInBackground,
};

export const Tools = {
  renderMemeList,
  renderMemeStatistics,
  MemeSortBy,
  MemeStatisticsType,
  ImageOperations: {
    inspect,
    flipHorizontal,
    flipVertical,
    rotate,
    resize,
    crop,
    grayscale,
    invert,
    mergeHorizontal,
    mergeVertical,
    gifSplit,
    gifMerge,
    gifReverse,
    gifChangeDuration,
  },
};
