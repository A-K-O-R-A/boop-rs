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
    id: "text.remove_newlines",
    name: "Remove Newlines",
    author: "AKORA",
    version: "1.0.0",
    tags: ["text", "newlines"],
    description: "This plugin removes all newline",
    inputType: "text",
    outputType: "text",
  };
}

/**
 * Run this plugin
 * @param {string} state
 * @returns {string}
 */
function run(state) {
  return state.replace(/(\n|  )/g, "");
}
