<template>
  <div class="desktop-downloads">
    <!-- Header with Stats -->
    <div class="downloads-header">
      <div class="header-content">
        <div class="header-text">
          <h1 class="page-title">Downloads</h1>
          <div class="stats-row">
            <div class="stat-item">
              <div class="stat-value">{{ activeDownloads.length }}</div>
              <div class="stat-label">Active</div>
            </div>
            <div class="stat-item">
              <div class="stat-value">{{ completedDownloads.length }}</div>
              <div class="stat-label">Completed</div>
            </div>
            <div class="stat-item">
              <div class="stat-value">{{ totalDownloads }}</div>
              <div class="stat-label">Total</div>
            </div>
          </div>
        </div>
        <div class="header-actions">
          <button @click="clearCompleted" class="btn btn-secondary">
            <Trash2 /> Clear Completed
          </button>
          <button @click="openDownloadsFolder" class="btn btn-primary">
            <Folder /> Open Folder
          </button>
        </div>
      </div>
    </div>
    
    <!-- Web App Connection Status -->
    <div class="connection-card" :class="{ connected: isWebAppConnected }">
      <div class="connection-content">
        <div class="connection-icon">
          <CloudDone v-if="isWebAppConnected" />
          <CloudQueue v-else />
        </div>
        <div class="connection-info">
          <h3>{{ isWebAppConnected ? 'Web App Connected' : 'Connect to Web App' }}</h3>
          <p>{{ isWebAppConnected ? `Syncing with ${webAppUrl}` : 'Access your library and sync downloads' }}</p>
        </div>
        <button @click="toggleWebAppConnection" class="btn" :class="isWebAppConnected ? 'btn-secondary' : 'btn-primary'">
          {{ isWebAppConnected ? 'Disconnect' : 'Connect' }}
        </button>
      </div>
    </div>
    
    <!-- Universal URL Input -->
    <div class="url-input-section">
      <div class="section-header">
        <h2>Quick Download</h2>
        <p>Download from Spotify, Apple Music, YouTube, Tidal, and more</p>
      </div>
      <div class="url-input-container">
        <div class="input-group">
          <div class="input-wrapper">
            <Link class="input-icon" />
            <input 
              v-model="urlInput"
              @keyup.enter="startDownload"
              type="text"
              placeholder="Enter URL from any music platform..."
              class="url-input"
            />
          </div>
          <select v-model="selectedQuality" class="quality-select">
            <option value="lossless">Lossless</option>
            <option value="high">High Quality</option>
            <option value="medium">Medium Quality</option>
            <option value="low">Low Quality</option>
          </select>
          <button @click="startDownload" class="btn btn-primary download-btn" :disabled="!urlInput.trim()">
            <Download /> Download
          </button>
        </div>
      </div>
    </div>
    
    <!-- Download Tabs -->
    <div class="download-tabs">
      <button 
        v-for="tab in tabs" 
        :key="tab.key"
        @click="activeTab = tab.key"
        class="tab-btn"
        :class="{ active: activeTab === tab.key }"
      >
        <component :is="tab.icon" />
        {{ tab.label }}
        <span v-if="tab.count > 0" class="tab-badge">{{ tab.count }}</span>
      </button>
    </div>
    
    <!-- Active Downloads -->
    <div v-if="activeTab === 'active' && activeDownloads.length > 0" class="downloads-section">
      <div class="downloads-grid">
        <div 
          v-for="download in activeDownloads" 
          :key="download.id"
          class="download-card active"
        >
          <div class="download-header">
            <div class="download-info">
              <div class="download-cover">
                <img v-if="download.cover" :src="download.cover" :alt="download.title" />
                <Music v-else class="placeholder-icon" />
              </div>
              <div class="download-details">
                <h4 class="download-title">{{ download.title }}</h4>
                <p class="download-artist">{{ download.artist }}</p>
                <div class="download-meta">
                  <span class="quality-badge">{{ download.quality }}</span>
                  <span class="size">{{ download.size }}</span>
                </div>
              </div>
            </div>
            <div class="download-actions">
              <button @click="pauseDownload(download.id)" class="btn btn-sm btn-secondary">
                <Pause />
              </button>
              <button @click="cancelDownload(download.id)" class="btn btn-sm btn-ghost">
                <X />
              </button>
            </div>
          </div>
          
          <div class="download-progress">
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: download.progress + '%' }"></div>
            </div>
            <div class="progress-info">
              <span class="progress-text">{{ download.progress }}%</span>
              <span class="speed">{{ download.speed }}</span>
              <span class="eta">{{ download.eta }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Completed Downloads -->
    <div v-if="activeTab === 'completed' && completedDownloads.length > 0" class="downloads-section">
      <div class="downloads-grid">
        <div 
          v-for="download in completedDownloads" 
          :key="download.id"
          class="download-card completed"
        >
          <div class="download-header">
            <div class="download-info">
              <div class="download-cover">
                <img v-if="download.cover" :src="download.cover" :alt="download.title" />
                <Music v-else class="placeholder-icon" />
              </div>
              <div class="download-details">
                <h4 class="download-title">{{ download.title }}</h4>
                <p class="download-artist">{{ download.artist }}</p>
                <div class="download-meta">
                  <span class="quality-badge">{{ download.quality }}</span>
                  <span class="size">{{ download.size }}</span>
                  <span class="date">{{ download.date }}</span>
                </div>
              </div>
            </div>
            <div class="download-actions">
              <button @click="playDownload(download.id)" class="btn btn-sm btn-primary">
                <Play />
              </button>
              <button @click="openFile(download.id)" class="btn btn-sm btn-secondary">
                <FolderOpen />
              </button>
              <button @click="deleteDownload(download.id)" class="btn btn-sm btn-ghost">
                <Trash2 />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Empty State -->
    <div v-if="totalDownloads === 0" class="empty-state">
      <div class="empty-content">
        <div class="empty-icon">
          <Download />
        </div>
        <h2>No Downloads Yet</h2>
        <p>Start downloading music from URLs or connect to your web app</p>
        <div class="empty-actions">
          <button @click="focusUrlInput" class="btn btn-primary">
            <Link /> Add URL
          </button>
          <button @click="openWebAppConnection" class="btn btn-secondary">
            <CloudQueue /> Connect Web App
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

// Reactive state
const urlInput = ref('')
const selectedQuality = ref('high')
const activeTab = ref('active')
const downloads = ref([])
const isWebAppConnected = ref(false)
const webAppUrl = ref('')

// Computed properties
const activeDownloads = computed(() => 
  downloads.value.filter(d => d.status === 'downloading' || d.status === 'paused')
)

const completedDownloads = computed(() => 
  downloads.value.filter(d => d.status === 'completed')
)

const totalDownloads = computed(() => downloads.value.length)

const tabs = computed(() => [
  { key: 'active', label: 'Active', icon: 'Download', count: activeDownloads.value.length },
  { key: 'completed', label: 'Completed', icon: 'CheckCircle', count: completedDownloads.value.length },
  { key: 'all', label: 'All', icon: 'List', count: totalDownloads.value }
])

// Methods
const startDownload = async () => {
  if (!urlInput.value.trim()) return
  
  try {
    const downloadId = await invoke('download_universal_url', {
      request: {
        url: urlInput.value,
        quality: selectedQuality.value,
        output_dir: '/downloads'
      }
    })
    
    // Add to downloads list
    downloads.value.push({
      id: downloadId,
      url: urlInput.value,
      title: 'Loading...',
      artist: 'Loading...',
      quality: selectedQuality.value,
      progress: 0,
      status: 'downloading',
      size: '0 MB',
      speed: '0 KB/s',
      eta: 'Calculating...',
      cover: null
    })
    
    urlInput.value = ''
  } catch (error) {
    console.error('Failed to start download:', error)
  }
}

const pauseDownload = async (id: string) => {
  try {
    await invoke('pause_download', { downloadId: id })
    updateDownloadStatus(id, 'paused')
  } catch (error) {
    console.error('Failed to pause download:', error)
  }
}

const cancelDownload = async (id: string) => {
  try {
    await invoke('cancel_download', { downloadId: id })
    downloads.value = downloads.value.filter(d => d.id !== id)
  } catch (error) {
    console.error('Failed to cancel download:', error)
  }
}

const playDownload = (id: string) => {
  // Play the downloaded file
  console.log('Playing download:', id)
}

const openFile = async (id: string) => {
  const download = downloads.value.find(d => d.id === id)
  if (download?.filePath) {
    await invoke('open_in_file_manager', { filePath: download.filePath })
  }
}

const deleteDownload = (id: string) => {
  downloads.value = downloads.value.filter(d => d.id !== id)
}

const clearCompleted = async () => {
  try {
    await invoke('clear_completed_downloads')
    downloads.value = downloads.value.filter(d => d.status !== 'completed')
  } catch (error) {
    console.error('Failed to clear completed downloads:', error)
  }
}

const openDownloadsFolder = async () => {
  await invoke('open_in_file_manager', { filePath: '/downloads' })
}

const toggleWebAppConnection = () => {
  if (isWebAppConnected.value) {
    disconnectFromWebApp()
  } else {
    connectToWebApp()
  }
}

const connectToWebApp = async () => {
  try {
    const connected = await invoke('connect_to_web_app', {
      url: webAppUrl.value,
      pairingCode: 'DEMO-CODE'
    })
    
    if (connected) {
      isWebAppConnected.value = true
    }
  } catch (error) {
    console.error('Failed to connect to web app:', error)
  }
}

const disconnectFromWebApp = async () => {
  try {
    await invoke('disconnect_from_web_app')
    isWebAppConnected.value = false
  } catch (error) {
    console.error('Failed to disconnect from web app:', error)
  }
}

const focusUrlInput = () => {
  const input = document.querySelector('.url-input') as HTMLInputElement
  input?.focus()
}

const updateDownloadStatus = (id: string, status: string) => {
  const download = downloads.value.find(d => d.id === id)
  if (download) {
    download.status = status
  }
}

// Lifecycle
onMounted(() => {
  // Load existing downloads
  loadDownloads()
  
  // Set up progress listener
  setupProgressListener()
})

const loadDownloads = async () => {
  try {
    const history = await invoke('get_download_history')
    downloads.value = history
  } catch (error) {
    console.error('Failed to load downloads:', error)
  }
}

const setupProgressListener = async () => {
  // Listen for download progress updates
  // This would use Tauri's event system
}
</script>

<style scoped>
@import './styles/desktop-app.css';

.desktop-downloads {
  max-width: 1200px;
  margin: 0 auto;
  padding: var(--spacing-6);
}

/* Header */
.downloads-header {
  margin-bottom: var(--spacing-8);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: var(--spacing-6);
}

.header-text h1 {
  font-size: var(--font-size-4xl);
  font-weight: var(--font-weight-bold);
  color: var(--on-surface);
  margin: 0 0 var(--spacing-4) 0;
}

.stats-row {
  display: flex;
  gap: var(--spacing-6);
}

.stat-item {
  text-align: center;
}

.stat-value {
  font-size: var(--font-size-2xl);
  font-weight: var(--font-weight-bold);
  color: var(--primary);
  line-height: 1;
}

.stat-label {
  font-size: var(--font-size-sm);
  color: var(--on-surface-variant);
  margin-top: var(--spacing-1);
}

.header-actions {
  display: flex;
  gap: var(--spacing-3);
}

/* Connection Card */
.connection-card {
  background: var(--surface-variant);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-6);
  margin-bottom: var(--spacing-8);
  transition: all var(--transition-base);
}

.connection-card.connected {
  background: var(--color-success);
  border-color: var(--color-success);
  color: var(--color-on-accent);
}

.connection-content {
  display: flex;
  align-items: center;
  gap: var(--spacing-4);
}

.connection-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--surface);
  border-radius: var(--radius-lg);
  color: var(--primary);
}

.connection-card.connected .connection-icon {
  background: rgba(255, 255, 255, 0.2);
  color: white;
}

.connection-info {
  flex: 1;
}

.connection-info h3 {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  margin: 0 0 var(--spacing-1) 0;
}

.connection-info p {
  font-size: var(--font-size-sm);
  opacity: 0.8;
  margin: 0;
}

/* URL Input Section */
.url-input-section {
  margin-bottom: var(--spacing-8);
}

.section-header h2 {
  font-size: var(--font-size-2xl);
  font-weight: var(--font-weight-semibold);
  color: var(--on-surface);
  margin: 0 0 var(--spacing-2) 0;
}

.section-header p {
  font-size: var(--font-size-base);
  color: var(--on-surface-variant);
  margin: 0 0 var(--spacing-4) 0;
}

.url-input-container {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-6);
}

.input-group {
  display: flex;
  gap: var(--spacing-3);
  align-items: stretch;
}

.input-wrapper {
  flex: 1;
  position: relative;
  display: flex;
  align-items: center;
}

.input-icon {
  position: absolute;
  left: var(--spacing-3);
  width: 20px;
  height: 20px;
  color: var(--on-surface-variant);
  z-index: 1;
}

.url-input {
  flex: 1;
  padding: var(--spacing-3) var(--spacing-3) var(--spacing-3) var(--spacing-10);
  border: 1px solid var(--border);
  border-radius: var(--radius-base);
  background: var(--background);
  color: var(--on-background);
  font-size: var(--font-size-base);
  transition: all var(--transition-fast);
}

.url-input:focus {
  outline: none;
  border-color: var(--primary);
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

.quality-select {
  padding: 0 var(--spacing-3);
  border: 1px solid var(--border);
  border-radius: var(--radius-base);
  background: var(--background);
  color: var(--on-background);
  font-size: var(--font-size-sm);
  min-width: 120px;
  cursor: pointer;
}

.download-btn {
  white-space: nowrap;
}

/* Tabs */
.download-tabs {
  display: flex;
  gap: var(--spacing-1);
  margin-bottom: var(--spacing-6);
  border-bottom: 1px solid var(--border);
}

.tab-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  padding: var(--spacing-3) var(--spacing-4);
  border: none;
  background: transparent;
  color: var(--on-surface-variant);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  cursor: pointer;
  transition: all var(--transition-fast);
  border-bottom: 2px solid transparent;
  position: relative;
}

.tab-btn:hover {
  color: var(--on-surface);
  background: var(--surface-variant);
}

.tab-btn.active {
  color: var(--primary);
  border-bottom-color: var(--primary);
}

.tab-badge {
  background: var(--primary);
  color: var(--on-primary);
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-bold);
  padding: var(--spacing-1) var(--spacing-2);
  border-radius: var(--radius-full);
  min-width: 20px;
  text-align: center;
}

/* Downloads Grid */
.downloads-section {
  margin-bottom: var(--spacing-6);
}

.downloads-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
  gap: var(--spacing-4);
}

/* Download Card */
.download-card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-4);
  transition: all var(--transition-base);
}

.download-card:hover {
  box-shadow: var(--shadow-md);
  transform: translateY(-1px);
}

.download-card.active {
  border-color: var(--primary);
  box-shadow: 0 0 0 1px var(--primary), var(--shadow-sm);
}

.download-card.completed {
  border-color: var(--color-success);
}

.download-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: var(--spacing-3);
  margin-bottom: var(--spacing-4);
}

.download-info {
  display: flex;
  gap: var(--spacing-3);
  flex: 1;
  min-width: 0;
}

.download-cover {
  width: 60px;
  height: 60px;
  border-radius: var(--radius-base);
  overflow: hidden;
  flex-shrink: 0;
  background: var(--surface-variant);
  display: flex;
  align-items: center;
  justify-content: center;
}

.download-cover img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.placeholder-icon {
  width: 24px;
  height: 24px;
  color: var(--on-surface-variant);
}

.download-details {
  flex: 1;
  min-width: 0;
}

.download-title {
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-semibold);
  color: var(--on-surface);
  margin: 0 0 var(--spacing-1) 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.download-artist {
  font-size: var(--font-size-sm);
  color: var(--on-surface-variant);
  margin: 0 0 var(--spacing-2) 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.download-meta {
  display: flex;
  gap: var(--spacing-2);
  align-items: center;
}

.quality-badge {
  background: var(--primary);
  color: var(--on-primary);
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-medium);
  padding: var(--spacing-1) var(--spacing-2);
  border-radius: var(--radius-sm);
}

.size, .date {
  font-size: var(--font-size-xs);
  color: var(--on-surface-variant);
}

.download-actions {
  display: flex;
  gap: var(--spacing-1);
  flex-shrink: 0;
}

/* Progress */
.download-progress {
  border-top: 1px solid var(--divider);
  padding-top: var(--spacing-3);
}

.progress-bar {
  height: 6px;
  background: var(--surface-variant);
  border-radius: var(--radius-full);
  overflow: hidden;
  margin-bottom: var(--spacing-2);
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--color-gradient-start), var(--color-gradient-end));
  border-radius: var(--radius-full);
  transition: width var(--transition-base);
}

.progress-info {
  display: flex;
  justify-content: space-between;
  font-size: var(--font-size-xs);
  color: var(--on-surface-variant);
}

/* Empty State */
.empty-state {
  text-align: center;
  padding: var(--spacing-16) var(--spacing-6);
}

.empty-content {
  max-width: 400px;
  margin: 0 auto;
}

.empty-icon {
  width: 80px;
  height: 80px;
  margin: 0 auto var(--spacing-6);
  background: var(--surface-variant);
  border-radius: var(--radius-2xl);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--on-surface-variant);
}

.empty-content h2 {
  font-size: var(--font-size-2xl);
  font-weight: var(--font-weight-semibold);
  color: var(--on-surface);
  margin: 0 0 var(--spacing-3) 0;
}

.empty-content p {
  font-size: var(--font-size-base);
  color: var(--on-surface-variant);
  margin: 0 0 var(--spacing-6) 0;
}

.empty-actions {
  display: flex;
  gap: var(--spacing-3);
  justify-content: center;
}

/* Responsive Design */
@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    align-items: stretch;
  }
  
  .stats-row {
    justify-content: center;
  }
  
  .input-group {
    flex-direction: column;
  }
  
  .downloads-grid {
    grid-template-columns: 1fr;
  }
  
  .download-header {
    flex-direction: column;
    align-items: stretch;
  }
  
  .download-actions {
    justify-content: flex-end;
  }
}
</style>
