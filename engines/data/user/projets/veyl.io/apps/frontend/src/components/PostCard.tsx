import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { PostHit } from '@/lib/api';
import { ExternalLink, Image, Video, Layout, Heart, MessageCircle, Eye, TrendingUp, UserPlus } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';
import { fr } from 'date-fns/locale';

interface PostCardProps {
  post: PostHit;
  onAddToWatch?: () => void;
}

export function PostCard({ post, onAddToWatch }: PostCardProps) {
  const formatNumber = (num?: number) => {
    if (!num) return '0';
    if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
    if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
    return num.toString();
  };

  const relativeTime = post.posted_at
    ? formatDistanceToNow(new Date(post.posted_at), { addSuffix: true, locale: fr })
    : '';

  const mediaIcon = post.media_type === 'VIDEO' ? <Video className="h-3 w-3" /> :
                   post.media_type === 'CAROUSEL_ALBUM' ? <Layout className="h-3 w-3" /> :
                   <Image className="h-3 w-3" />;

  return (
    <Card className="overflow-hidden transition-smooth hover:shadow-glow hover:border-primary/50">
      {/* Media */}
      <div className="relative aspect-square overflow-hidden rounded-t-lg bg-muted">
        {post.media_url || post.thumbnail_url ? (
          <img
            src={post.thumbnail_url || post.media_url}
            alt={post.caption || 'Post media'}
            className="h-full w-full object-cover"
            loading="lazy"
          />
        ) : (
          <div className="flex h-full items-center justify-center">
            <Image className="h-12 w-12 text-muted-foreground" />
          </div>
        )}
        <div className="absolute top-2 left-2 flex gap-2">
          <Badge variant="secondary" className="gap-1">
            {mediaIcon}
            {post.media_type || 'IMAGE'}
          </Badge>
          {post.score_trend !== undefined && post.score_trend > 0 && (
            <Badge variant="secondary" className="gap-1" title="Score de tendance">
              <TrendingUp className="h-3 w-3" />
              {post.score_trend.toFixed(1)}
            </Badge>
          )}
        </div>
      </div>

      {/* Content */}
      <div className="p-4 space-y-3">
        <div className="flex items-start justify-between gap-2">
          <div className="flex-1 min-w-0">
            {post.username && (
              <p className="font-semibold truncate">@{post.username}</p>
            )}
            <p className="text-xs text-muted-foreground" title={post.posted_at}>
              {relativeTime}
            </p>
          </div>
        </div>

        {post.caption && (
          <p className="text-sm line-clamp-3 text-muted-foreground">{post.caption}</p>
        )}

        {post.hashtags && post.hashtags.length > 0 && (
          <div className="flex flex-wrap gap-1">
            {post.hashtags.slice(0, 4).map((tag, i) => (
              <Badge key={i} variant="outline" className="text-xs cursor-pointer hover:bg-primary/10">
                #{tag}
              </Badge>
            ))}
            {post.hashtags.length > 4 && (
              <Badge variant="outline" className="text-xs">
                +{post.hashtags.length - 4}
              </Badge>
            )}
          </div>
        )}

        <div className="flex items-center gap-3 text-sm text-muted-foreground pt-2 border-t border-border">
          {post.like_count !== undefined && (
            <div className="flex items-center gap-1" title="Likes">
              <Heart className="h-4 w-4" />
              <span>{formatNumber(post.like_count)}</span>
            </div>
          )}
          {post.comment_count !== undefined && (
            <div className="flex items-center gap-1" title="Commentaires">
              <MessageCircle className="h-4 w-4" />
              <span>{formatNumber(post.comment_count)}</span>
            </div>
          )}
          {post.view_count !== undefined && (
            <div className="flex items-center gap-1" title="Vues">
              <Eye className="h-4 w-4" />
              <span>{formatNumber(post.view_count)}</span>
            </div>
          )}
        </div>

        <div className="flex gap-2 pt-2">
          <Button
            size="sm"
            variant="outline"
            className="flex-1 gap-2"
            asChild
          >
            <a
              href={post.permalink}
              target="_blank"
              rel="noopener noreferrer"
              aria-label="Ouvrir sur Instagram"
            >
              <ExternalLink className="h-4 w-4" />
              Ouvrir sur Instagram
            </a>
          </Button>
          {onAddToWatch && (
            <Button
              size="sm"
              onClick={onAddToWatch}
              className="gap-2"
              aria-label="Ajouter à ma veille"
            >
              <UserPlus className="h-4 w-4" />
              Suivre
            </Button>
          )}
        </div>
      </div>
    </Card>
  );
}
