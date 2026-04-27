package com.irina.myfirstgame.interfaces;

import com.irina.myfirstgame.objects.ScoreEntry;
import java.util.List;

/**
 * Interface pour le stockage et le chargement des scores.
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public interface ScoreStorage {
    List<ScoreEntry> load();
    void save(List<ScoreEntry> entries);
}

