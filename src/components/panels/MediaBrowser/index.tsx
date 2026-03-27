import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useTimelineStore } from '../../stores/useTimelineStore';

interface MediaBrowserProps {
  // We could pass in specific props, but for now we'll use the store and invoke
}

export const MediaBrowser: React.FC<MediaBrowserProps> = () => {
  const { addClip } = useTimelineStore();
  const [currentPath, setCurrentPath] = useState<string>('/');
  const [entries, setEntries] = useState<Array<any>>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [previewPath, setPreviewPath] = useState<string | null>(null);
  const [previewDuration, setPreviewDuration] = useState<number | null>(null);
  const [hoveredEntry, setHoveredEntry] = useState<string | null>(null);

  // Load directory contents
  const loadDirectory = async (path: string) => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke('read_directory', { path });
      setEntries(result);
      setCurrentPath(path);
    } catch (err: any) {
      setError(err?.message || 'Unknown error');
      console.error('Failed to read directory:', err);
    } finally {
      setLoading(false);
    }
  };

  // Go up one directory
  const goUp = async () => {
    if (currentPath === '/') return;
    const parent = currentPath.split('/').slice(0, -1).join('/') || '/';
    await loadDirectory(parent);
  };

  // Handle hover for preview
  const handleHover = async (path: string, isDir: boolean) => {
    if (isDir) {
      setPreviewPath(null);
      setPreviewDuration(null);
      return;
    }
    setHoveredEntry(path);
    try {
      const metadata = await invoke('probe_media', { path });
      setPreviewPath(path);
      setPreviewDuration(metadata.duration);
    } catch (err) {
      console.error('Failed to probe media:', err);
      setPreviewPath(null);
      setPreviewDuration(null);
    }
  };

  // Handle leaving hover
  const handleLeaveHover = () => {
    setHoveredEntry(null);
    setPreviewPath(null);
    setPreviewDuration(null);
  };

  // Handle double click to play preview (we'll just set the preview state for now)
  const handleDoubleClick = async (path: string, isDir: boolean) => {
    if (isDir) return;
    try {
      const metadata = await invoke('probe_media', { path });
      setPreviewPath(path);
      setPreviewDuration(metadata.duration);
      // In a real app, we'd start playing the preview in a separate panel
    } catch (err) {
      console.error('Failed to play preview:', err);
    }
  };

  useEffect(() => {
    loadDirectory(currentPath);
  }, [currentPath]);

  if (loading && entries.length === 0) {
    return <div className="mediabrowser-panel">Loading...</div>;
  }

  if (error) {
    return <div className="mediabrowser-panel">Error: {error}</div>;
  }

  return (
    <div className="mediabrowser-panel">
      <div className="mediabrowser-header">
        <button onClick={goUp} className="nav-button">
          ↑ Up
        </button>
        <div className="current-path">{currentPath}</div>
      </div>
      <div className="mediabrowser-content">
        <div className="file-list">
          {entries.map((entry: any) => (
            <div
              key={entry.path}
              className={`file-entry ${entry.is_dir ? 'directory' : 'file'} ${hoveredEntry === entry.path ? 'hovered' : ''}`}
              onDragEnter={(e) => {
                e.preventDefault();
                e.dataTransfer.dropEffect = 'copy';
                // We'll set the dragged data for drop
                e.dataTransfer.setData('text/plain', JSON.stringify({
                  path: entry.path,
                  is_dir: entry.is_dir,
                  duration: entry.duration
                }));
              }}
              onDragOver={(e) => {
                e.preventDefault();
                e.dataTransfer.dropEffect = 'copy';
              }}
              onMouseEnter={() => handleHover(entry.path, entry.is_dir)}
              onMouseLeave={handleLeaveHover}
              onDoubleClick={() => handleDoubleClick(entry.path, entry.is_dir)}
            >
              <div className="file-icon">
                {entry.is_dir ? '📁' : '📄'}
              </div>
              <div className="file-info">
                <div className="file-name">{entry.name}</div>
                {!entry.is_dir && entry.duration !== undefined && (
                  <div className="file-duration">
                    {Math.floor(entry.duration / 60)}:{('0' + Math.floor(entry.duration % 60)).slice(-2)}
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
        {/* Preview panel */}
        {previewPath && (
          <div className="mediabrowser-preview">
            <div className="preview-header">
              <div className="preview-name">
                {previewPath.split(/[\\/]/).pop() || 'unknown'}
              </div>
              {previewDuration !== null && (
                <div className="preview-duration">
                  {Math.floor(previewDuration / 60)}:{('0' + Math.floor(previewDuration % 60)).slice(-2)}
                )
              )}
            </div>
            <div className="preview-body">
              {/* In a real app, we'd show a video thumbnail or first frame here */}
              <div className="preview-placeholder">
                Preview
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
