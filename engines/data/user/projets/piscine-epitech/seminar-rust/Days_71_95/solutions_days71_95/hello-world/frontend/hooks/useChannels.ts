"use client";

import { useState, useEffect, useCallback } from "react";
import {
  listChannels,
  createChannel as apiCreateChannel,
  getChannel as apiGetChannel,
  updateChannel as apiUpdateChannel,
  deleteChannel as apiDeleteChannel,
  Channel,
} from "@/lib/api-server";
import { handleAuthError, isAuthError, getErrorMessage } from "@/lib/auth/utils";

export function useChannels(serverId: string | null) {
  const [channels, setChannels] = useState<Channel[]>([]);
  const [selectedChannel, setSelectedChannel] = useState<Channel | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadChannels = useCallback(async (id: string) => {
    try {
      setLoading(true);
      setError(null);
      const data = await listChannels(id);
      setChannels(data);
      if (data.length > 0) {
        setSelectedChannel(data[0]);
      } else {
        setSelectedChannel(null);
      }
    } catch (err) {
      const errorMessage = getErrorMessage(err, "Failed to load channels");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return;
      }
      setError(errorMessage);
      setChannels([]);
      setSelectedChannel(null);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (serverId) {
      loadChannels(serverId);
    } else {
      setChannels([]);
      setSelectedChannel(null);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [serverId]);

  const selectChannel = useCallback((channel: Channel | null) => {
    setSelectedChannel(channel);
  }, []);

  const createChannel = useCallback(async (name: string): Promise<Channel | null> => {
    if (!serverId) return null;
    
    try {
      setError(null);
      const newChannel = await apiCreateChannel(serverId, name);
      await loadChannels(serverId);
      setSelectedChannel(newChannel);
      return newChannel;
    } catch (err) {
      const errorMessage = getErrorMessage(err, "Failed to create channel");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return null;
      }
      setError(errorMessage);
      return null;
    }
  }, [serverId, loadChannels]);

  const getChannel = useCallback(async (id: string): Promise<Channel | null> => {
    try {
      setError(null);
      const channel = await apiGetChannel(id);
      // Mettre à jour le channel dans la liste si présent
      setChannels((prev) =>
        prev.map((c) => (c.id === id ? channel : c))
      );
      return channel;
    } catch (err) {
      const errorMessage = getErrorMessage(err, "Failed to get channel");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return null;
      }
      setError(errorMessage);
      return null;
    }
  }, []);

  const updateChannel = useCallback(async (id: string, name: string): Promise<Channel | null> => {
    if (!serverId) return null;
    
    let previousChannels: Channel[] = [];
    
    try {
      setError(null);
      
      // Optimisme UI : mise à jour immédiate
      setChannels((prev) => {
        previousChannels = [...prev];
        return prev.map((c) => (c.id === id ? { ...c, name, updated_at: new Date().toISOString() } : c));
      });
      
      // Si c'est le channel sélectionné, le mettre à jour aussi
      setSelectedChannel((prev) => {
        if (prev?.id === id) {
          return { ...prev, name, updated_at: new Date().toISOString() };
        }
        return prev;
      });

      const updatedChannel = await apiUpdateChannel(id, name);
      
      // Mise à jour avec les données du serveur
      setChannels((prev) =>
        prev.map((c) => (c.id === id ? updatedChannel : c))
      );
      
      setSelectedChannel((prev) => {
        if (prev?.id === id) {
          return updatedChannel;
        }
        return prev;
      });

      return updatedChannel;
    } catch (err) {
      // Rollback en cas d'erreur
      if (previousChannels.length > 0) {
        setChannels(previousChannels);
        const previousChannel = previousChannels.find((c) => c.id === id);
        if (previousChannel && selectedChannel?.id === id) {
          setSelectedChannel(previousChannel);
        }
      }
      
      const errorMessage = getErrorMessage(err, "Failed to update channel");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return null;
      }
      setError(errorMessage);
      return null;
    }
  }, [serverId, selectedChannel]);

  const deleteChannel = useCallback(async (id: string): Promise<boolean> => {
    if (!serverId) return false;
    
    let previousChannels: Channel[] = [];
    let previousSelected: Channel | null = null;
    const wasSelected = selectedChannel?.id === id;
    
    try {
      setError(null);
      
      // Optimisme UI : suppression immédiate
      setChannels((prev) => {
        previousChannels = [...prev];
        return prev.filter((c) => c.id !== id);
      });
      
      // Si c'était le channel sélectionné, sélectionner le premier disponible
      if (wasSelected) {
        previousSelected = selectedChannel;
        setChannels((prev) => {
          const remaining = prev.filter((c) => c.id !== id);
          setSelectedChannel(remaining.length > 0 ? remaining[0] : null);
          return prev;
        });
      }

      await apiDeleteChannel(id);
      
      // Si c'était le channel sélectionné, mettre à jour la sélection
      if (wasSelected) {
        setChannels((prev) => {
          const remaining = prev.filter((c) => c.id !== id);
          setSelectedChannel(remaining.length > 0 ? remaining[0] : null);
          return prev;
        });
      }
      
      return true;
    } catch (err) {
      // Rollback en cas d'erreur
      if (previousChannels.length > 0) {
        setChannels(previousChannels);
        if (wasSelected && previousSelected) {
          setSelectedChannel(previousSelected);
        }
      }
      
      const errorMessage = getErrorMessage(err, "Failed to delete channel");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return false;
      }
      setError(errorMessage);
      return false;
    }
  }, [serverId, selectedChannel]);

  return {
    channels,
    selectedChannel,
    loading,
    error,
    selectChannel,
    createChannel,
    getChannel,
    updateChannel,
    deleteChannel,
  };
}

