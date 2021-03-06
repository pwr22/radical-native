/* eslint-disable @typescript-eslint/no-explicit-any */

let bundleURL: string;

const handleFromBundleMessage = (message: any): void => {
  switch (message.method) {
    case "ready":
      window.postMessage(
        {
          type: "bundle",
          target: "page",
          method: "init",
          bundle: bundleURL,
        },
        "*"
      );
      break;
  }
};

const handleMessage = async (message: any): Promise<void> => {
  const rpcReply: { [key: string]: any } = {
    type: message.type,
    target: "page",
    method: "rpc",
    rpcId: message.rpcId,
  };

  try {
    rpcReply.reply = await browser.runtime.sendMessage(message);
  } catch (error) {
    console.error(error.toString(), message);
    rpcReply.error = error.toString();
  }

  window.postMessage(rpcReply, "*");
};

window.addEventListener("message", function (event) {
  if (event.source !== window || event?.data?.target !== "contentscript") {
    return;
  }

  switch (event.data.type) {
    case "bundle":
      handleFromBundleMessage(event.data);
      break;

    default:
      handleMessage(event.data);
      break;
  }
});

browser.runtime.onMessage.addListener(
  async (message: any): Promise<any> => {
    if (message.method === "ready") {
      bundleURL = message.bundle;
      return "ready";
    }
  }
);
