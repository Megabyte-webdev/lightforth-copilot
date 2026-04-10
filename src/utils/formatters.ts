export const extractErrorMessage = (error: any) => {
  try {
    // 1. Dig into the response data (where FastAPI's detail lives)
    const resData = error?.response?.data;

    if (resData) {
      // Handle the specific structure: { detail: { message: "..." } }
      if (resData.detail?.message) {
        return resData.detail.message;
      }

      // Handle FastAPI Validation Errors: { detail: [{ loc: [...], msg: "..." }] }
      if (Array.isArray(resData.detail)) {
        return resData.detail
          .map((d: { loc: string | any[]; msg: string }) => {
            const field = Array.isArray(d.loc)
              ? d.loc[d.loc.length - 1]
              : d.loc || "";
            const formattedField = field
              .toString()
              .replace(/_/g, " ")
              .replace(/^\w/, (c: string) => c.toUpperCase());
            const cleanMsg = d.msg
              ?.replace("Field required", "is required")
              .trim();
            return formattedField ? `${formattedField} ${cleanMsg}` : cleanMsg;
          })
          .join(" | ");
      }

      // Handle simple string detail: { detail: "Error message" }
      if (typeof resData.detail === "string") {
        return resData.detail;
      }

      // Handle direct message or error keys: { message: "..." }
      if (resData.message && typeof resData.message === "string")
        return resData.message;
      if (resData.error && typeof resData.error === "string")
        return resData.error;
    }

    // 2. If no response data, check for standard Axios error messages
    if (error?.message) {
      if (error.message.includes("network error"))
        return "Network error. Please check your connection.";
      if (error.message.includes("timeout"))
        return "Request timed out. Please try again.";
      return error.message;
    }

    return "An unknown error occurred";
  } catch (err) {
    console.error("Error parsing message:", err);
    return "Something went wrong while processing the error.";
  }
};
