<script setup lang="ts">
import { onMounted, ref } from "vue";
import { add, get_replacement_img } from "drmwasm";

const image = ref();

const readFile = async (file: File) => {
  const promise = new Promise<ArrayBuffer>((resolve, reject) => {
    const file_reader = new FileReader();
    file_reader.onload = () => {
      const file_content = file_reader.result;

      if (file_content) {
        resolve(file_content as ArrayBuffer);
      } else {
        reject("Cannot read file content");
      }
    };
    file_reader.readAsArrayBuffer(file);
  });

  return promise;
};

const drawImage = (file_content: Uint8Array) => {
  if (image.value) {
    const image_elem = image.value as HTMLImageElement;
    const file_blob = new Blob([file_content], { type: "image/jpeg" });
    const file_url = URL.createObjectURL(file_blob);

    image_elem.src = file_url;
  }
};

const handleFile = async (event: Event) => {
  const target = event.target as HTMLInputElement;
  const files = target.files;

  if (files && files.length > 0) {
    const first_file = files.item(0);

    if (first_file) {
      const file_content = await readFile(first_file);
      const file_uint8 = new Uint8Array(file_content);
      const subimage_replacement = get_replacement_img(file_uint8);

      console.log(subimage_replacement);
      drawImage(file_uint8);
    }
  }
};

onMounted(() => {
  console.log(add);
  console.log(add(20));
  console.log(get_replacement_img(new Uint8Array()));
});
</script>

<template>
  <div>
    <div>
      <input type="file" accept="image/*" @change="handleFile" />
      <img class="preview" ref="image" />
    </div>
  </div>
</template>

<style scoped>
.preview {
  width: auto;
  height: 800px;
}
</style>
