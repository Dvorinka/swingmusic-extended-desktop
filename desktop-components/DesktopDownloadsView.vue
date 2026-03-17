<template>
  <div class="desktop-downloads">
    <!-- Download Queue -->
    <div class="downloads-header">
      <h2>Downloads</h2>
      <div class="header-actions">
        <button @click="clearCompleted" class="btn-secondary">
          <Trash2 /> Clear Completed
        </button>
        <button @click="openDownloadsFolder" class="btn-primary">
          <Folder /> Open Folder
        </button>
      </div>
    </div>
    
    <!-- Universal URL Input -->
    <div class="url-input-section">
      <h3>Quick Download</h3>
      <div class="url-input-container">
        <input 
          v-model="urlInput"
          @keyup.enter="startDownload"
          type="text"
          placeholder="Enter URL from Spotify, Apple Music, YouTube, Tidal, etc..."
          class="url-input"
        />
        <select v-model="selectedQuality" class="quality-select">
          <option value="lossless">Lossless</option>
          <option value="high">High Quality</option>
          <option value="medium">Medium Quality</option>
          <option value="low">Low Quality</option>
        </select>
        <button @click="startDownload" class="btn-primary download-btn">
          <Download /> Download
        </button>
      </div>
    </div>
    
    <!-- Active Downloads -->
    <div class="active-downloads" v-if="activeDownloads.length > 0">
      <h3>Active Downloads</h3>
      <div class="download-list">
        <div 
          v-for="download in activeDownloads" 
          :key="download.id"
          class="download-item active"
        >
          <div class="download-info">
            <div class="download-cover-placeholder">
              <Music />
            </div>
            <div class="download-details">
              <h4 class="download-title">{{ download.id }}</h4>
              <p class="download-quality">{{ selectedQuality }}</p>
              <p class="download-status">{{ download.status }}</p>
            </div>
          </div>
          
          <div class="download-progress">
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :style="{ width: download.progress + '%' }"
              ></div>
            </div>
            <span class="progress-text">{{ Math.round(download.progress) }}%</span>
          </div>
          
          <div class="download-actions">
            <button 
              v-if="download.status === 'downloading'"
              @click="pauseDownload(download.id)"
              class="btn-icon"
            >
              <Pause />
            </button>
            <button 
              v-else-if="download.status === 'paused'"
              @click="resumeDownload(download.id)"
              class="btn-icon"
            >
              <Play />
            </button>
            <button 
              @click="cancelDownload(download.id)"
              class="btn-icon danger"
            >
              <X />
            </button>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Completed Downloads -->
    <div class="completed-downloads" v-if="completedDownloads.length > 0">
      <h3>Completed Downloads</h3>
      <div class="download-list">
        <div 
          v-for="download in completedDownloads" 
          :key="download.id"
          class="download-item completed"
        >
          <div class="download-info">
            <div class="download-cover-placeholder">
              <CheckCircle />
            </div>
            <div class="download-details">
              <h4 class="download-title">{{ download.id }}</h4>
              <p class="download-quality">{{ download.file_path || 'Completed' }}</p>
              <p class="download-status">Completed</p>
            </div>
          </div>
          
          <div class="download-status">
            <CheckCircle class="status-icon success" />
            <span class="status-text">Completed</span>
          </div>
          
          <div class="download-actions">
            <button 
              @click="openFileLocation(download.file_path)"
              class="btn-icon"
              v-if="download.file_path"
            >
              <Folder />
            </button>
            <button 
              @click="removeDownload(download.id)"
              class="btn-icon"
            >
              <Trash2 />
            </button>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Empty State -->
    <div v-if="activeDownloads.length === 0 && completedDownloads.length === 0" class="empty-state">
      <Download class="empty-icon" />
      <h3>No downloads yet</h3>
      <p>Start downloading music from Universal sources to see it here</p>
      <button @click="focusUrlInput" class="btn-primary">
        <Download /> Start Downloading
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'

// Reactive state
const urlInput = ref('')
const selectedQuality = ref('high')
const activeDownloads = ref([])
const completedDownloads = ref([])

onMounted(async () => {
  await loadDownloads()
  await setupEventListeners()
})

onUnmounted(() => {
  // Cleanup
})

const loadDownloads = async () => {
  try {
    const history = await invoke('get_download_history')
    
    // Separate active and completed downloads
    activeDownloads.value = history.filter((d: any) => 
      d.status === 'downloading' || d.status === 'paused'
    )
    completedDownloads.value = history.filter((d: any) => 
      d.status === 'completed'
    )
  } catch (error) {
    console.error('Failed to load downloads:', error)
  }
}

const setupEventListeners = async () => {
  // Listen for download progress updates
  await listen('download-progress', (event) => {
    const progress = event.payload as any
    updateDownloadProgress(progress)
  })
}

const updateDownloadProgress = (progress: any) => {
  // Update active downloads
  const activeIndex = activeDownloads.value.findIndex((d: any) => d.id === progress.id)
  if (activeIndex !== -1) {
    activeDownloads.value[activeIndex] = progress
  }
  
  // Move to completed if finished
  if (progress.status === 'completed') {
    const index = activeDownloads.value.findIndex((d: any) => d.id === progress.id)
    if (index !== -1) {
      const completed = activeDownloads.value.splice(index, 1)[0]
      completedDownloads.value.unshift(completed)
    }
  }
}

const startDownload = async () => {
  if (!urlInput.value.trim()) return
  
  try {
    const downloadRequest = {
      url: urlInput.value.trim(),
      quality: selectedQuality.value,
      output_dir: '/downloads' // This should come from settings
    }
    
    const downloadId = await invoke('download_universal_url', { 
      request: downloadRequest 
    })
    
    // Add to active downloads
    activeDownloads.value.push({
      id: downloadId,
      progress: 0,
      status: 'downloading',
      file_path: null,
      error: null
    })
    
    // Clear input
    urlInput.value = ''
    
  } catch (error) {
    console.error('Failed to start download:', error)
  }
}

const pauseDownload = async (downloadId: string) => {
  try {
    await invoke('pause_download', { downloadId })
    await loadDownloads()
  } catch (error) {
    console.error('Failed to pause download:', error)
  }
}

const resumeDownload = async (downloadId: string) => {
  try {
    await invoke('resume_download', { downloadId })
    await loadDownloads()
  } catch (error) {
    console.error('Failed to resume download:', error)
  }
}

const cancelDownload = async (downloadId: string) => {
  try {
    await invoke('cancel_download', { downloadId })
    await loadDownloads()
  } catch (error) {
    console.error('Failed to cancel download:', error)
  }
}

const removeDownload = async (downloadId: string) => {
  // Remove from local state
  const index = completedDownloads.value.findIndex((d: any) => d.id === downloadId)
  if (index !== -1) {
    completedDownloads.value.splice(index, 1)
  }
}

const clearCompleted = async () => {
  try {
    await invoke('clear_completed_downloads')
    completedDownloads.value = []
  } catch (error) {
    console.error('Failed to clear completed downloads:', error)
  }
}

const openFileLocation = async (filePath: string) => {
  try {
    await invoke('open_in_file_manager', { filePath })
  } catch (error) {
    console.error('Failed to open file location:', error)
  }
}

const openDownloadsFolder = async () => {
  try {
    // This should get the download directory from settings
    await invoke('open_in_file_manager', { filePath: '/downloads' })
  } catch (error) {
    console.error('Failed to open downloads folder:', error)
  }
}

const focusUrlInput = () => {
  // Focus the URL input field
  const input = document.querySelector('.url-input') as HTMLInputElement
  if (input) {
    input.focus()
  }
}
</script>

<style scoped>
.desktop-downloads {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.downloads-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.downloads-header h2 {
  font-size: 24px;
  font-weight: 600;
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 12px;
}

.url-input-section {
  background: var(--background-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  padding: 20px;
}

.url-input-section h3 {
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 16px 0;
}

.url-input-container {
  display: flex;
  gap: 12px;
  align-items: center;
}

.url-input {
  flex: 1;
  padding: 12px;
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  background: var(--background-primary);
  color: var(--text-primary);
  font-size: 14px;
}

.url-input:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.quality-select {
  padding: 12px;
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  background: var(--background-primary);
  color: var(--text-primary);
  font-size: 14px;
  min-width: 120px;
}

.download-btn {
  padding: 12px 20px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.active-downloads,
.completed-downloads {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.active-downloads h3,
.completed-downloads h3 {
  font-size: 18px;
  font-weight: 600;
  margin: 0;
}

.download-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.download-item {
  background: var(--background-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  padding: 16px;
  display: flex;
  align-items: center;
  gap: 16px;
  transition: all 0.2s;
}

.download-item:hover {
  border-color: var(--border-secondary);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.download-item.active {
  border-left: 4px solid var(--accent-primary);
}

.download-item.completed {
  border-left: 4px solid var(--accent-success);
}

.download-info {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
  flex: 1;
}

.download-cover-placeholder {
  width: 48px;
  height: 48px;
  border-radius: 4px;
  background: var(--background-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-tertiary);
}

.download-details {
  min-width: 0;
  flex: 1;
}

.download-title {
  font-size: 14px;
  font-weight: 500;
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.download-quality {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.download-status {
  font-size: 11px;
  color: var(--text-tertiary);
  margin: 0;
  text-transform: uppercase;
  font-weight: 500;
}

.download-progress {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  min-width: 80px;
}

.progress-bar {
  width: 80px;
  height: 6px;
  background: var(--background-tertiary);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--accent-primary);
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 11px;
  color: var(--text-secondary);
  font-weight: 500;
}

.download-status {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 100px;
}

.status-icon {
  width: 16px;
  height: 16px;
}

.status-icon.success {
  color: var(--accent-success);
}

.status-text {
  font-size: 12px;
  color: var(--text-secondary);
}

.download-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.btn-icon {
  border: none;
  background: transparent;
  cursor: pointer;
  padding: 8px;
  border-radius: 4px;
  transition: background-color 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-icon:hover {
  background: var(--background-tertiary);
}

.btn-icon.danger:hover {
  background: var(--accent-error);
  color: white;
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  gap: 16px;
}

.empty-icon {
  width: 64px;
  height: 64px;
  color: var(--text-tertiary);
}

.empty-state h3 {
  font-size: 20px;
  font-weight: 600;
  margin: 0;
  color: var(--text-secondary);
}

.empty-state p {
  font-size: 14px;
  color: var(--text-tertiary);
  margin: 0;
}

.btn-primary {
  background: var(--accent-primary);
  color: var(--accent-on-primary);
  border: none;
  padding: 12px 20px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: background-color 0.2s;
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn-primary:hover {
  background: var(--accent-primary-hover);
}

.btn-secondary {
  background: var(--background-tertiary);
  color: var(--text-primary);
  border: none;
  padding: 12px 20px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: background-color 0.2s;
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn-secondary:hover {
  background: var(--background-quaternary);
}
</style>
