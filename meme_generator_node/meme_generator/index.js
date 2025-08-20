const native = require("./js-binding.js");

module.exports = {
  Meme: native.Meme,
  getMeme: native.getMeme,
  getMemes: native.getMemes,
  getMemeKeys: native.getMemeKeys,
  searchMemes: native.searchMemes,
  getVersion: native.getVersion,
  Resources: {
    checkResources: native.checkResources,
    checkResourcesInBackground: native.checkResourcesInBackground,
  },
  Tools: {
    renderMemeList: native.renderMemeList,
    renderMemeStatistics: native.renderMemeStatistics,
    MemeSortBy: native.MemeSortBy,
    MemeStatisticsType: native.MemeStatisticsType,
    ImageOperations: {
      inspect: native.inspect,
      flipHorizontal: native.flipHorizontal,
      flipVertical: native.flipVertical,
      rotate: native.rotate,
      resize: native.resize,
      crop: native.crop,
      grayscale: native.grayscale,
      invert: native.invert,
      mergeHorizontal: native.mergeHorizontal,
      mergeVertical: native.mergeVertical,
      gifSplit: native.gifSplit,
      gifMerge: native.gifMerge,
      gifReverse: native.gifReverse,
      gifChangeDuration: native.gifChangeDuration,
    },
  },
};
