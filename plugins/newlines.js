/**
 * @typedef Metadata
 * @type {object}
 * @property {string} id - the if of this plugin
 * @property {string} name - the name of this plugin
 * @property {string} description - the description of this plugin
 * @property {string} inputType - the text input type this plugin accepts
 * @property {string} inputType - the text output type of this plugin
 */

/** @returns {Metadata} */
function metadata() {
  return {
    id: "general.remove_newlines",
    name: "Remove Newlines",
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
