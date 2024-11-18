document.getElementById("fileInput").addEventListener("change", handleFileSelect);

async function handleFileSelect(event) {
  const file = event.target.files[0];
  if (!file || file.type !== "text/plain") {
    alert("Please select a valid .txt file.");
    return;
  }

  // Create FormData to send the file
  const formData = new FormData();
  formData.append("file", file);

  try {
    // Send the file to the backend
    const response = await fetch("http://127.0.0.1:3030/upload", {
      method: "POST",
      body: formData,
    });

    if (!response.ok) {
      throw new Error(`Failed to upload file: ${response.statusText}`);
    }

    // Parse the JSON response
    const wordFrequencies = await response.json();

    // Clear the custom-div and add a heading
    const div = document.querySelector(".custom-div");
    div.innerHTML = ""; // Remove the heading and clear previous content

    // Ensure wordFrequencies is an array
    if (Array.isArray(wordFrequencies) && wordFrequencies.length > 0) {
      wordFrequencies.forEach(item => {
        const p = document.createElement("p");
        p.textContent = `${item.word}: ${item.count}`;// Display word and count vertically
        div.appendChild(p);
      });
    } else {
      div.innerHTML += "<p>No words found in the file.</p>";
    }
  } catch (error) {
    console.error("Error uploading file:", error);
    alert("Error uploading file. Check the console for details.");
  }
}
