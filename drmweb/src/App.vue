<script setup lang="ts">
import { onMounted, ref } from "vue";
import { add, get_replacement_jpeg, get_replacement_webp } from "drmwasm";

const canvas = ref();
const file_meta = ref("");
const watermark_x = ref(-1);
const watermark_y = ref(-1);
const watermark_w = ref(-1);
const watermark_h = ref(-1);
const original_img = ref<Blob | null>(null);
const replacement_img = ref<Blob | null>(null);

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

const renderCanvas = async () => {
  if (!original_img.value) return;
  if (!canvas.value) return;

  const canvas_elem = canvas.value as HTMLCanvasElement;
  const canvas_context = canvas_elem.getContext("2d");
  const file_blob = original_img.value;
  const file_bitmap = await createImageBitmap(file_blob);

  if (canvas_context) {
    canvas_context.drawImage(
      file_bitmap,
      0,
      0,
      file_bitmap.width,
      file_bitmap.height,
      0,
      0,
      600,
      800
    );

    const image_scale_w = file_bitmap.width / 600;
    const image_scale_h = file_bitmap.height / 800;

    if (replacement_img.value) {
      const replacement_blob = replacement_img.value;
      const replacement_bitmap = await createImageBitmap(replacement_blob);
      const replacement_width = watermark_w.value / image_scale_w;
      const replacement_height = watermark_h.value / image_scale_h;
      const replacement_x = watermark_x.value / image_scale_w;
      const replacement_y = watermark_y.value / image_scale_h;

      console.log(image_scale_w, image_scale_h, replacement_x);

      canvas_context.drawImage(replacement_bitmap, replacement_x, replacement_y, replacement_width, replacement_height)
    }
  }
};

const storeReplacementImage = (subimage_replacement: any, metadata: string) => {
  const subimage = subimage_replacement.real_img;
  const subimage_arr = new Uint8Array(subimage);
  const subimage_blob = new Blob([subimage_arr], { type: metadata });

  watermark_h.value = subimage_replacement.height;
  watermark_w.value = subimage_replacement.width;
  watermark_x.value = subimage_replacement.x;
  watermark_y.value = subimage_replacement.y;
  replacement_img.value = subimage_blob;
};

const clearReplacementImage = () => {
  watermark_h.value = -1;
  watermark_w.value = -1;
  watermark_x.value = -1;
  watermark_y.value = -1;
  replacement_img.value = null;
};

const handleFile = async (event: Event) => {
  const target = event.target as HTMLInputElement;
  const files = target.files;

  if (files && files.length > 0) {
    const first_file = files.item(0);

    if (first_file) {
      const metadata = first_file.type;
      const file_content = await readFile(first_file);
      const file_uint8 = new Uint8Array(file_content);
      const file_blob = new Blob([file_content], { type: metadata });

      file_meta.value = metadata;
      original_img.value = file_blob;

      try {
        if (metadata == "image/jpeg") {
          const subimage_replacement = get_replacement_jpeg(file_uint8);
          storeReplacementImage(subimage_replacement, metadata);
        } else {
          const subimage_replacement = get_replacement_webp(file_uint8);
          storeReplacementImage(subimage_replacement, metadata);
        }
      } catch (err) {
        console.log(err);
        clearReplacementImage();
      }

      await renderCanvas();
    }
  }
};

onMounted(() => {
  console.log(add(20));
});
</script>

<template>
  <div>
    <div>
      <input type="file" accept="image/jpeg,image/webp" @change="handleFile" />
      <canvas width="600" height="800" class="preview" ref="canvas"></canvas>
    </div>
  </div>
</template>

<style scoped>
.preview {
  width: 600px;
  height: 800px;
}
</style>
