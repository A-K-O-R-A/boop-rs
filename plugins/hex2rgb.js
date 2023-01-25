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
    id: "color.hex2rgb",
    name: "Hex to RGB",
    author: "AKORA",
    version: "1.0.0",
    tags: ["rgb", "hex", "convert", "color"],
    description: "Convert color from Hex to RGB format",
    inputType: "text",
    outputType: "text",
  };
}

const reg = /#[0-9a-fA-F]{6}/;

/**
 *
 * @param {string} hex
 * @returns {string}
 */
function run(hex) {
  if (!reg.test(hex)) throw "Invalid HEX color";

  hex = hex.slice(1); // Remove '#'

  try {
    let r = parseInt(hex.slice(0, 2), 16);
    let g = parseInt(hex.slice(2, 4), 16);
    let b = parseInt(hex.slice(4, 6), 16);

    return `${r},${g},${b}`;
  } catch (e) {
    throw "Invalid RGB value" + e;
  }
}
