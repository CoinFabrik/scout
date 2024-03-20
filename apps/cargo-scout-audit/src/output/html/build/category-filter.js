document.addEventListener("DOMContentLoaded", () => {
  const buttons = document.querySelectorAll(".category");
  buttons.forEach((button) => {
    button.addEventListener("click", () => {
      const category = button.getAttribute("data-category");
      const categorySections = document.querySelectorAll(".category-section");
      categorySections.forEach((section) => {
        if (
          category === "all" ||
          section.getAttribute("data-category") === category
        ) {
          section.classList.remove("hidden");
        } else {
          section.classList.add("hidden");
        }
      });
    });
  });
});
