# hashtags/schemas.py
from pydantic import BaseModel, Field
from typing import Optional
from datetime import datetime

class HashtagBase(BaseModel):
    name: str
    platform_id: int
    last_scraped: Optional[datetime] = None

class HashtagCreate(HashtagBase):
    pass

class HashtagUpdate(BaseModel):
    name: Optional[str] = None
    platform_id: Optional[int] = None
    last_scraped: Optional[datetime] = None

class HashtagResponse(HashtagBase):
    id: int
    updated_at: Optional[datetime] = None
    
    class Config:
        from_attributes = True
