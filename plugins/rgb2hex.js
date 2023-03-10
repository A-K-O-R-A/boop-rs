/**
 * @typedef Metadata
 * @type {object}
 * @property {string} id - the if of this plugin
 * @property {string} author - the author of this plugin
 * @property {string} version - the version of this plugin (Please update if you chaneg somethign and make a PR)
 * @property {string[]} tags - Tags that will be used for the search
 * @property {string} name - the name of this plugin
 * @property {string} description - the description of this plugin
 * @property {string} inputType - the text input type this plugin accepts
 * @property {string} outputType - the text output type of this plugin
 */

/** @returns {Metadata} */
function metadata() {
  return {
    id: "color.rgb2hex",
    name: "RGB to Hex",
    author: "AKORA",
    version: "1.0.0",
    tags: ["rgb", "hex", "convert", "color"],
    description: "Convert color from RGB to Hex format",
    inputType: "text",
    outputType: "text",
  };
}

const reg = /(rgb\()?(\d{1,3}(,| )){2}\d{1,3}(\)?)/;

/**
 *
 * @param {string} rgb
 * @returns {string}
 */
function run(rgb) {
  if (!reg.test(rgb)) throw "Invalid RGB format";
  rgb = rgb.replace("rgb(", "").replace(")").replaceAll(" ", ",");

  const rgbArray = rgb.split(",");

  if (rgbArray.length !== 3) throw "Invalid RGB format";

  try {
    let hex =
      "#" +
      rgbArray.map((c) => parseInt(c).toString(16).padStart(2, "0")).join("");
    return hex.toUpperCase();
  } catch (_) {
    return error("Invalid RGB value");
  }
}
